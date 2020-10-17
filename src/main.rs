#[macro_use]
extern crate lazy_static;

use env_logger::{Builder, Env};
use log::{error, info, warn};
use std::net::SocketAddr;
use std::io::Write;
use tokio::net::{TcpListener, TcpStream};

mod config;
mod networking;

use networking::Connection;

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

async fn process(socket: TcpStream, address: SocketAddr) {
    let mut conn = Connection::new(address);
}

#[tokio::main]
async fn main() {
    init_logging();
    init_config();

    let listener = TcpListener::bind(&config::CONFIG.listen_address).await.unwrap();
    info!("Listening on {}", listener.local_addr().unwrap());

    loop {
        let (socket, address) = listener.accept().await.unwrap();
        process(socket, address).await;
    }
}
