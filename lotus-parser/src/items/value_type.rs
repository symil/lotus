use parsable::parsable;
use crate::program::ProgramContext;

use super::{GenericArguments, Identifier, TypeSuffix};

#[parsable]
pub struct ValueType {
    pub name: Identifier,
    pub generics: GenericArguments
}

impl ValueType {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Type> {
        
    }
}