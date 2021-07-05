use std::{collections::{HashSet}};

pub struct WriteBuffer {
    bytes: Vec<u8>,
    addresses: HashSet<usize>
}

pub enum RegisterResult {

}

impl WriteBuffer {
    pub fn new() -> Self {
        Self {
            bytes: vec![],
            addresses: HashSet::new()
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        self.bytes.push(byte);
    }

    pub fn write(&mut self, bytes: &[u8]) {
        self.bytes.extend_from_slice(bytes);
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }

    pub fn register(&mut self, addr: usize) -> bool {
        self.addresses.insert(addr)
    }
}