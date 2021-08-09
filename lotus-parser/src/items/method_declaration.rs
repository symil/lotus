use parsable::parsable;
use crate::program::{ProgramContext, Wasm};
use super::{FunctionDeclaration, FunctionSignature, Identifier, MethodCondition, MethodQualifier, Statement, StructDeclaration, VarPath};

#[parsable]
pub struct MethodDeclaration {
    pub qualifier: Option<MethodQualifier>,
    pub name: Identifier,
    #[parsable(brackets="[]", separator=",")]
    pub conditions: Vec<MethodCondition>,
    pub signature: Option<FunctionSignature>,
    #[parsable(brackets="{}")]
    pub statements: Vec<Statement>
}

impl MethodDeclaration {
    pub fn process_signature(&self, owner: &StructDeclaration, context: &mut ProgramContext) {
        todo!()
    }

    pub fn process_body(&self, owner: &StructDeclaration, context: &mut ProgramContext) -> Option<Wasm> {
        todo!()
    }
}