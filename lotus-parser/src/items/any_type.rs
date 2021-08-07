use parsable::parsable;
use super::{FunctionType, Identifier, TypeSuffix, ValueType};

#[parsable]
pub struct FullType {
    pub item: ItemType,
    pub suffix: Option<TypeSuffix>
}

#[parsable]
pub enum ItemType {
    Value(ValueType),
    Function(FunctionType)
}