#[macro_use]
extern crate log;

fn main() {
    // for this example we need to use init_named as the example name does not match the packagge name from env!
    zarthus_env_logger::init_named("long_running_app::app");

    app::run();
}

mod app {
    pub fn run() {
        task::quick::instant("Opening diary");
        config::load();

        task::slow::delayed("Hanging laundry", 2000);
        task::slow::delayed("Eating dinner", 1000);
        task::slow::delayed("Going to bed", 1000);
        task::slow::delayed("Brush teeth", 500);
        task::slow::delayed("Waking up", 500);

        task::quick::instant("Starting robots");
        std::thread::spawn(|| {
            task::slow::delayed("Robot 1 is going to work", 1000);
        });
        std::thread::spawn(|| {
            task::slow::delayed("Robot 2 is folding laundry", 1500);
        });
        std::thread::spawn(|| {
            task::slow::delayed("Robot 3 is cleaning", 1800);
        });

        task::slow::delayed("Eating breakfast", 1000);

        task::slow::delayed("Going to work", 1000);

        task::quick::instant("Closing diary");
        config::save();
    }

    mod config {
        pub fn load() {
            debug!("Loading config");
        }

        pub fn save() {
            debug!("Saving config");
        }
    }

    mod task {
        pub fn outcome(name: &str) {
            let seconds_since_epoch = std::time::UNIX_EPOCH.elapsed().unwrap().as_secs();
            match seconds_since_epoch % 3 {
                0 => {
                    info!("Task {} completed", name);
                }
                1 => {
                    warn!("Task {} experienced problems", name);
                }
                2 => {
                    error!("Task {} failed", name);
                }
                _ => {}
            }
        }

        pub mod quick {
            use crate::app::task::outcome;

            pub fn instant(name: &str) {
                info!("Starting instant task {}", name);
                outcome(name);
            }
        }

        pub mod slow {
            use crate::app::task::outcome;

            pub fn delayed(name: &str, sleeps_for: u64) {
                info!("Starting delayed task {}", name);
                std::thread::sleep(std::time::Duration::from_millis(sleeps_for));
                outcome(name);
            }
        }
    }
}
