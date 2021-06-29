pub struct WriteBuffer<'a> {
    bytes: &'a mut [u8],
    cursor: usize
}

impl<'a> WriteBuffer<'a> {
    pub fn new(bytes: &'a mut [u8]) -> Self {
        Self {
            bytes,
            cursor: 0
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        self.bytes[self.cursor] = byte;
        self.cursor += 1;
    }

    pub fn write(&mut self, bytes: &[u8]) {
        self.bytes[self.cursor..self.cursor+bytes.len()].copy_from_slice(bytes);
        self.cursor += bytes.len();
    }

    pub fn as_bytes(self) -> &'a [u8] {
        &self.bytes[0..self.cursor]
    }
}