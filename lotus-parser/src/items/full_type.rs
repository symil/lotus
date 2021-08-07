use parsable::parsable;
use super::{ItemType, TypeSuffix};

#[parsable]
pub struct FullType {
    pub item: ItemType,
    pub suffix: Option<TypeSuffix>
}