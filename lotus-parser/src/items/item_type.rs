use parsable::parsable;
use super::{FunctionType, ValueType};

#[parsable]
pub enum ItemType {
    Value(ValueType),
    Function(FunctionType)
}