pub trait Serializable : Sized {
    fn write_bytes(value: &Self, bytes: &mut Vec<u8>);
    fn read_bytes(bytes: &[u8]) -> Option<(Self, usize)>;
    // TODO: read_unchecked method?

    fn serialize(&self) -> Vec<u8> {
        let mut bytes : Vec<u8> = vec![];
        Self::write_bytes(self, &mut bytes);
        bytes
    }

    fn deserialize(bytes: &[u8]) -> Option<Self> {
        match Self::read_bytes(bytes) {
            None => None,
            Some((value, _)) => Some(value)
        }
    }
}

pub use lotus_serializable_derive::Serializable;