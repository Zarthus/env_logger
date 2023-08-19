#[macro_use]
extern crate log;

use std::time::Duration;

fn main() {
    // for this example we need to use init_named as the example name does not match the packagge name from env!
    zarthus_env_logger::init_named("modular_init");

    debug!("root level debug");
    example::call_log_funcs();
    std::thread::sleep(Duration::new(1, 0));
    error!("root level error");
}

mod example {
    pub fn call_log_funcs() {
        debug!("Calling log fuctions from within example::call_log_funcs!");
        foo::a();
        bar::b();
    }

    mod foo {
        pub fn a() {
            info!("Info log from within example::foo!");
        }
    }

    mod bar {
        pub fn b() {
            warn!("Warn log from within example::bar!");
        }
    }
}
