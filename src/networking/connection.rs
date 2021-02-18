use super::packet::{utils, Packet};
use std::net::SocketAddr;
use thiserror::Error;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

pub struct Connection {
    socket: TcpStream,
    pub address: SocketAddr,
}

impl Connection {
    pub fn new(socket: TcpStream, address: SocketAddr) -> Self {
        Connection { socket, address }
    }

    async fn fetch_varint(&mut self) -> Result<(i32, Vec<u8>), PacketParseError> {
        let mut buf = [0; 5];

        let read_res = self.socket.read_exact(&mut buf).await;
        if read_res.is_err() {
            return Err(PacketParseError::BadVarInt(Some(read_res.unwrap_err())));
        }

        return match utils::read_varint(&buf) {
            Ok((i, s)) => Ok((i, buf[s..].to_owned())),
            Err(_) => return Err(PacketParseError::BadVarInt(None)),
        };
    }

    pub async fn get_packet(&mut self) -> Result<Packet, PacketParseError> {
        let mut length = self.fetch_varint().await?;

        println!("l:{}", length.0 as usize);

        let mut buf = Box::new(Vec::with_capacity(length.0 as usize));
        buf.append(&mut length.1);

        println!("b:{:?}", buf);

        let read_res = self.socket.read_exact(&mut buf).await;

        println!("b2:{:?}", buf);

        if read_res.is_err() {
            return Err(PacketParseError::BadData(Some(read_res.unwrap_err())));
        }

        return match utils::read_chunk(buf, length.0).await {
            Ok(d) => {
                let id = d.0;
                let data = d.1;

                Ok(Packet::new(length.0, id, data))
            }
            Err(_) => Err(PacketParseError::BadData(None)),
        };
    }
}

#[derive(Error, Debug)]
pub enum PacketParseError {
    #[error("Expected VarInt, got something else!")]
    BadVarInt(Option<std::io::Error>),
    #[error("Expected VarLong, got something else!")]
    BadVarLong(Option<std::io::Error>),
    #[error("Couldn't parse in packet data.")]
    BadData(Option<std::io::Error>),
    #[error("Unknown parsing error.")]
    Unknown(),
}
