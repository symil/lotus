#[derive(Debug, Clone)]
pub enum ValueType {
    Void,
    Generic(String),
    Type(TypeRef),
    TypeRef(TypeRef)
}

#[derive(Debug, Clone)]
pub struct TypeRef {
    pub id: u64,
    pub name: String
}