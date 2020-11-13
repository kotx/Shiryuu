use std::fmt;

pub mod utils;

pub struct Packet {
    length: i32,
    id: i32,
    data: Box<Vec<i8>>
}

impl Packet {
    pub fn new(length: i32, id: i32, data: Box<Vec<i8>>) -> Self {
        Packet {
            length,
            id,
            data
        }
    }
}

impl fmt::Debug for Packet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Packet")
            .field("id", &format!("0x{:x}", self.id))
            .field("length", &self.length)
            .field("data", &self.data)
            .finish()
    }
}
