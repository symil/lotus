use parsable::parsable;
use super::{FunctionType, Identifier, ValueType};

#[parsable]
pub enum AnyType {
    Value(ValueType),
    Function(FunctionType)
}