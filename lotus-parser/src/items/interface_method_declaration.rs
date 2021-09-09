use parsable::parsable;
use crate::program::{ProgramContext, Type};
use super::{FunctionSignature, Identifier};

#[parsable]
pub struct InterfaceMethodDeclaration {
    pub name: Identifier,
    pub signature: FunctionSignature
}

impl InterfaceMethodDeclaration {
    pub fn process(&self, context: &mut ProgramContext) -> (Identifier, Vec<Type>, Option<Type>) {
        let (arguments, return_type) = self.signature.process(context);
        let argument_types = arguments.into_iter().map(|(name, ty)| ty).collect();
        let name = self.name.clone();

        (name, argument_types, return_type)
    }
}