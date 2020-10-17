use std::net::SocketAddr;

pub struct Connection {
    address: SocketAddr,
}

impl Connection {
    pub fn new(address: SocketAddr) -> Self {
        Connection {
            address,
        }
    }
}
