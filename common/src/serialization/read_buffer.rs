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

    pub fn read(&mut self, length: usize) -> Option<&[u8]> {
        if self.cursor + length > self.bytes.len() {
            None
        } else {
            let start = self.cursor;
            let end = self.cursor + length;

            self.cursor = end;

            Some(&self.bytes[start..end])
        }
    }

    pub fn read_unchecked(&mut self, length: usize) -> &[u8] {
        let start = self.cursor;
        let end = self.cursor + length;

        self.cursor = end;

        &self.bytes[start..end]
    }
}