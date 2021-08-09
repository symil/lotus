use parsable::parsable;

use crate::program::{ProgramContext, Wasm};

use super::{FunctionSignature, Identifier, Statement, FullType};

#[parsable]
pub struct FunctionDeclaration {
    #[parsable(prefix="fn")]
    pub name: Identifier,
    pub signature: FunctionSignature,
    #[parsable(brackets="{}")]
    pub statements: Vec<Statement>
}

impl FunctionDeclaration {
    pub fn pre_process(&self, context: &mut ProgramContext) {
        todo!()
    }

    pub fn process(&self, context: &mut ProgramContext) -> Option<Wasm> {
        todo!()
    }
}