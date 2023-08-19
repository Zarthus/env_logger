#[macro_use]
extern crate log;

fn main() {
    zarthus_env_logger::init_custom("custom_init", log::LevelFilter::Trace, "%H:%M:%S %Z");

    trace!("Trace log!");
    debug!("Debug log!");
    info!("Info log!");
    warn!("Warning log!");
    error!("Error log!");
}
