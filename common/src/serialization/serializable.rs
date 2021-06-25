use super::{write_buffer::WriteBuffer, read_buffer::ReadBuffer};

pub trait Serializable : Sized {
    fn write_bytes(value: &Self, buffer: &mut WriteBuffer);
    fn read_bytes(buffer: &mut ReadBuffer) -> Option<Self>;
    // TODO: read_unchecked method?

    fn serialize(&self) -> Vec<u8> {
        let mut buffer = WriteBuffer::new();
        
        Self::write_bytes(self, &mut buffer);

        buffer.into_bytes()
    }

    fn deserialize(bytes: &[u8]) -> Option<Self> {
        let mut buffer = ReadBuffer::new(bytes);

        Self::read_bytes(&mut buffer)
    }
}