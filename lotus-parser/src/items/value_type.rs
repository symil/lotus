use parsable::parsable;
use super::{Identifier, TypeSuffix};

#[parsable]
pub struct ValueType {
    pub name: Identifier,
}