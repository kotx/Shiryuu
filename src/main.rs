#[macro_use]
extern crate lazy_static;

use env_logger::{Builder, Env};
use log::{debug, error, info, trace, warn};
use std::net::SocketAddr;
use std::io::Write;
use tokio::net::{TcpListener, TcpStream};

mod config;
mod networking;

use networking::connection::{Connection, PacketParseError};

fn init_logging() {
    let env = Env::default().filter("SHIRYUU_LOG_LEVEL");

    Builder::from_env(env)
        .format(|buf, record| {
            let timestamp = buf.timestamp_seconds();
            writeln!(buf, "[{} {}]: {}", timestamp, record.level(), record.args())
        })
        .init();
}

fn init_config() -> Result<(), config::ConfigError> {
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

            return Ok(());
        }
        Err(e) => Err(e),
    }
}

async fn process(socket: TcpStream, address: SocketAddr) {
    info!("Incoming connection from {}", address);
    let mut connection = Connection::new(socket, address);
    let packet = match connection.get_packet().await {
        Ok(packet ) => packet,
        Err(PacketParseError::BadVarInt(e)) | Err(PacketParseError::BadVarLong(e)) => {
            info!("Dropped bad connection from {}; received bad VarInt or VarLong.", connection.address);
            debug!("Reason: {:?}", e);
            return;
        }
        Err(PacketParseError::BadData(e)) => {
            info!("Dropped bad connection from {}; received bad data.", connection.address);
            debug!("Reason: {:?}", e);
            return;
        }
        Err(PacketParseError::Unknown()) => {
            warn!("Unknown packet parsing error. Uh oh. Dropping connection from {}...", connection.address);
            return;
        }
    };

    trace!("Received packet from {}:\n{:?}", connection.address, packet);
}

#[tokio::main]
async fn main() {
    init_logging();
    match init_config() {
        Ok(_) => info!("Config is valid, proceeding."),
        Err(e) => return error!("{}", e)
    }

    let listener = TcpListener::bind(&config::CONFIG.listen_address).await.unwrap();
    info!("Listening on {}", listener.local_addr().unwrap());

    loop {
        let (socket, address) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            process(socket, address).await;
        });
    }
}
