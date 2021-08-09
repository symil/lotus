use parsable::parsable;
use crate::program::{ConstantAnnotation, ProgramContext, Wasm};
use super::VarDeclaration;

#[parsable]
pub struct ConstantDeclaration {
    pub var_declaration: VarDeclaration
}

impl ConstantDeclaration {
    pub fn pre_process(&self, context: &mut ProgramContext) {
        todo!()
    }

    pub fn process(&self, context: &mut ProgramContext) -> Option<Wasm> {
        todo!()
    }
}