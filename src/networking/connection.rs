use tokio::net::TcpStream;
use std::net::SocketAddr;

pub struct Connection {
    socket: TcpStream,
    address: SocketAddr
}

impl Connection {
    pub fn new(socket: TcpStream, address: SocketAddr) -> Self {
        Connection {
            socket,
            address
        }
    }
}
