#[macro_use]
extern crate log;

fn main() {
    zarthus_env_logger::init_named_many(vec!["many_names", "FOO", "BAR", "BAZ"]);

    trace!("Trace log!");
    debug!("Debug log!");
    info!("Info log!");
    warn!("Warning log!");
    error!("Error log!");
}
