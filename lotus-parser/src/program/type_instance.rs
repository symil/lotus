use super::ValueType;

#[derive(Debug, Clone)]
pub struct TypeInstance {
    pub id: u64,
    pub type_id: u64,
    pub generics: Vec<ValueType>
}