use std::{collections::{HashSet}};

pub struct WriteBuffer {
    bytes: Vec<u8>,
    addresses: HashSet<usize>
}

impl WriteBuffer {
    pub fn new() -> Self {
        Self {
            bytes: vec![],
            addresses: HashSet::new()
        }
    }

    pub fn get_index(&self) -> usize {
        self.bytes.len()
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

    // returns `true` if the address is registered for the first time
    pub fn register(&mut self, addr: usize) -> bool {
        self.addresses.insert(addr)
    }
}