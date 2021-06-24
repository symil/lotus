pub trait Request : Sized {
    fn serialize(value: &Self) -> Vec<u8>;
    fn deserialize(bytes: &[u8]) -> Option<Self>;
}