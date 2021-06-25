use super::read_buffer::ReadBuffer;

pub trait Serializable : Sized {
    fn write_bytes(value: &Self, bytes: &mut Vec<u8>);
    fn read_bytes(buffer: &mut ReadBuffer) -> Option<Self>;
    // TODO: read_unchecked method?

    fn serialize(&self) -> Vec<u8> {
        let mut bytes : Vec<u8> = vec![];
        Self::write_bytes(self, &mut bytes);
        bytes
    }

    fn deserialize(bytes: &[u8]) -> Option<Self> {
        let mut buffer = ReadBuffer::new(bytes);

        Self::read_bytes(&mut buffer)
    }
}