#[macro_use]
extern crate log;

fn main() {
    // for this example we need to use init_named as the example name does not match the packagge name from env!
    zarthus_env_logger::init_named("simple_init");

    trace!("Trace log!");
    debug!("Debug log!");
    info!("Info log!");
    warn!("Warning log!");
    error!("Error log!");
}
