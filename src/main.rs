#[macro_use]
extern crate lazy_static;

use env_logger::{Builder, Env};
use log::{error, info, warn};
use std::io::Write;

mod config;

fn init_logging() {
    let env = Env::default().filter("SHIRYUU_LOG_LEVEL");

    Builder::from_env(env)
        .format(|buf, record| {
            let timestamp = buf.timestamp_seconds();
            writeln!(buf, "[{} {}]: {}", timestamp, record.level(), record.args())
        })
        .init();
}

fn init_config() {
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
                warn!("{}", warning);
            }

            info!("Config is valid, proceeding.");
        }
        Err(e) => return error!("{}", e),
    }
}

fn main() {
    init_logging();
    init_config();

    info!("Binding to address {}", &config::CONFIG.address);
}
