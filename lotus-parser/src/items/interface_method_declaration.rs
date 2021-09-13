use parsable::parsable;
use crate::program::{ProgramContext, Type};
use super::{FunctionSignature, Identifier};

#[parsable]
pub struct InterfaceMethodDeclaration {
    pub name: Identifier,
    #[parsable(suffix=";")]
    pub signature: FunctionSignature
}

impl InterfaceMethodDeclaration {
    pub fn process(&self, context: &mut ProgramContext) -> (Identifier, Vec<(Identifier, Type)>, Option<Type>) {
        let (arguments, return_type) = self.signature.process(context);
        let name = self.name.clone();

        (name, arguments, return_type)
    }
}