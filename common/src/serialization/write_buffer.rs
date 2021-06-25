pub struct WriteBuffer {
    bytes: Vec<u8>
}

impl WriteBuffer {
    pub fn new() -> Self {
        Self {
            bytes: vec![],
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
}