use parsable::parsable;
use crate::program::{ProgramContext, Type};
use super::{ItemType, TypeSuffix};

#[parsable]
pub struct FullType {
    pub item: ItemType,
    pub suffix: Vec<TypeSuffix>
}

impl FullType {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Type> {

    }
}