use parsable::parsable;

use super::Identifier;

#[parsable]
pub enum Type {
    Value(ValueType),
    Function(FunctionType)
}

#[parsable]
pub struct FunctionType {
    #[parsable(brackets="()", separator=",")]
    pub arguments: Vec<Type>,
    #[parsable(prefix="()")]
    pub return_value: Option<Box<Type>>
}

#[parsable]
pub struct ValueType {
    pub name: Identifier,
    pub suffix: Option<TypeSuffix>
}

#[parsable]
pub enum TypeSuffix {
    Array = "[]"
}