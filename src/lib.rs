use env_logger::fmt::{Color, Formatter};
use log::{Level, LevelFilter, Record};
use std::env;

pub fn init() {
    init_custom(vec![env!("CARGO_PKG_NAME")], LevelFilter::Debug, "%H:%M:%S")
}

pub fn init_named(module_name: &'static str) {
    init_custom(vec![module_name], LevelFilter::Debug, "%H:%M:%S")
}

pub fn init_named_many(module_names: Vec<&'static str>) {
    init_custom(module_names, LevelFilter::Debug, "%H:%M:%S")
}

pub fn init_custom(
    module_names: Vec<&'static str>,
    level_filter: LevelFilter,
    timestamp_format: &'static str,
) {
    if std::env::var("RUST_LOG").is_ok() {
        env_logger::init();
    } else {
        builder(&module_names, level_filter, timestamp_format)
    }
}

/// Initializes the env_logger through a custom builder
///
/// params:
///   module_name: the name of the package
///   level_filter: the level to filter for your package
///   timestamp_format: if the `chrono` feature is enabeld, format timestamps like this
///
/// Does not return any value.
fn builder(
    module_names: &Vec<&'static str>,
    level_filter: LevelFilter,
    timestamp_format: &'static str,
) {
    let first_module_name = module_names.first().unwrap();

    let mut builder = env_logger::builder();

    for module_name in module_names {
        builder.filter_module(module_name, level_filter);
    }

    builder
        .filter(None, LevelFilter::Error)
        .format_level(true)
        .format(|buf, record| format(buf, record, first_module_name, timestamp_format))
        .init();

    log::trace!("RUST_LOG was not set, using self-built debug logger");
}

fn format(
    buf: &mut Formatter,
    record: &Record,
    module_name: &str,
    timestamp_format: &str,
) -> std::io::Result<()> {
    use std::io::Write;

    let _target = record.target();

    let mut style = buf.style();
    let level = match record.level() {
        Level::Trace => style.set_color(Color::Magenta).value("TRACE"),
        Level::Debug => style.set_color(Color::Blue).value("DEBUG"),
        Level::Info => style.set_color(Color::Green).value("INFO "),
        Level::Warn => style.set_color(Color::Yellow).value("WARN "),
        Level::Error => style.set_color(Color::Red).value("ERROR"),
    };

    let mut style = buf.style();
    let target = style.set_bold(true).value(record.target());

    let mut style = buf.style();
    let ts = buf.timestamp().to_string();
    let time = style
        .set_color(Color::Rgb(128, 128, 128))
        .value(format_timestamp(ts, timestamp_format));

    writeln!(
        buf,
        "{} {} {} > {}",
        time,
        level,
        target.to_string().replace(module_name, "@"),
        record.args()
    )
}

#[cfg(feature = "chrono")]
fn format_timestamp(ts: String, timestamp_format: &str) -> String {
    chrono::DateTime::parse_from_rfc3339(&ts)
        .map(|dt| dt.format(timestamp_format).to_string())
        .unwrap_or_else(|_| ts)
}

#[cfg(not(feature = "chrono"))]
fn format_timestamp(ts: String, _timestamp_format: &str) -> String {
    ts
}

#[cfg(test)]
mod tests {
    use crate::{builder, format_timestamp};

    #[test]
    fn test_builder() {
        let modules_names = vec!["foo"];

        builder(&modules_names, log::LevelFilter::Error, "%H:M:%S");
    }

    // #[test]
    // fn test_format() {
    //     let mut formatter: &mut Formatter;
    //
    //     let record = Record::builder()
    //         .level(log::Level::Info)
    //         .file(Some("foo"))
    //         .module_path(Some("modulepath"))
    //         .build();
    //
    //     let output = format(formatter, &record, "modulepath", "%H:%M")
    //         .expect("This should not have happened.");
    //
    //     assert_eq!("Foo", output)
    // }

    #[cfg(feature = "chrono")]
    #[test]
    fn test_format_timestamp() {
        assert_eq!(
            "20:05:05 2023 +02:00",
            format_timestamp("2023-12-31T20:05:05+02:00".to_string(), "%H:%M:%S %Y %Z"),
        );
    }

    #[cfg(feature = "chrono")]
    #[test]
    fn test_format_timestamp_crashes_on_bad_format() {
        assert_eq!("", format_timestamp("".to_string(), "%9 ABC"));
    }

    #[cfg(not(feature = "chrono"))]
    #[test]
    fn test_format_timestamp() {
        assert_eq!(
            "1000",
            format_timestamp("1000".to_string(), "%H:%M:%S %Y %Z"),
        );
    }
}
