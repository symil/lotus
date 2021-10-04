use parsable::parsable;
use crate::program::ProgramContext;

use super::{CompilerConstant, Identifier};

#[parsable]
pub enum FieldOrMethodName {
    CompilerConstant(CompilerConstant),
    Identifier(Identifier)
}

impl FieldOrMethodName {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Identifier> {
        match self {
            FieldOrMethodName::CompilerConstant(compiler_constant) => compiler_constant.process_as_name(context),
            FieldOrMethodName::Identifier(identifier) => Some(identifier.clone()),
        }
    }
}