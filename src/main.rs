#[macro_use]
extern crate lazy_static;

use log::{error, info, warn};

mod config;

fn main() {
    env_logger::init();

    info!(
        "{} {}",
        config::built_info::get_pretty_name(),
        config::built_info::PKG_VERSION
    );

    if !config::built_info::PKG_VERSION_PRE.is_empty() {
        info!(
            "This version of {} is in alpha. Proceed at your own risk.",
            config::built_info::get_pretty_name()
        );
    }

    info!("");

    let warnings = config::CONFIG.validate();
    match warnings {
        Ok(_) => {
            for warning in warnings.unwrap() {
                warn!("Warning: {}", warning);
            }

            info!("Config is valid, proceeding.");
        }
        Err(e) => return error!("Config: {}", e),
    }

    info!("Binding to address {}", &config::CONFIG.address);
}
