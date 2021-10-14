use parsable::parsable;
use crate::program::ProgramContext;

use super::{Macro, Identifier};

#[parsable]
pub enum FieldOrMethodName {
    Macro(Macro),
    Identifier(Identifier)
}

impl FieldOrMethodName {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Identifier> {
        match self {
            FieldOrMethodName::Macro(compiler_constant) => compiler_constant.process_as_name(context),
            FieldOrMethodName::Identifier(identifier) => Some(identifier.clone()),
        }
    }
}