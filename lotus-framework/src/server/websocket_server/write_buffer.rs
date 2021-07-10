pub struct WriteBuffer<'a> {
    bytes: &'a mut [u8],
    index: usize
}

impl<'a> WriteBuffer<'a> {
    pub fn new(bytes: &'a mut [u8]) -> Self {
        Self {
            bytes,
            index: 0
        }
    }

    #[allow(dead_code)]
    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn write_byte(&mut self, byte: u8) {
        self.bytes[self.index] = byte;
        self.index += 1;
    }

    pub fn write(&mut self, bytes: &[u8]) {
        self.bytes[self.index..self.index+bytes.len()].copy_from_slice(bytes);
        self.index += bytes.len();
    }

    pub fn as_bytes(self) -> &'a [u8] {
        &self.bytes[0..self.index]
    }
}