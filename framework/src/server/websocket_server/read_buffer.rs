pub struct ReadBuffer<'a> {
    bytes: &'a [u8],
    cursor: usize
}

impl<'a> ReadBuffer<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self {
            bytes,
            cursor: 0
        }
    }

    pub fn read_byte(&mut self) -> u8 {
        self.cursor += 1;
        self.bytes[self.cursor - 1]
    }

    pub fn read(&mut self, length: usize) -> &[u8] {
        let start = self.cursor;
        let end = self.cursor + length;

        self.cursor = end;

        &self.bytes[start..end]
    }

    pub fn read_as_array<const N : usize>(&mut self) -> [u8; N] {
        let mut arr : [u8; N] = [0; N];

        arr.copy_from_slice(self.read(N));

        arr
    }

    pub fn is_consumed(&self) -> bool {
        self.cursor >= self.bytes.len()
    }
}