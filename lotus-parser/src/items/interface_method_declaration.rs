use parsable::parsable;
use crate::program::{FieldKind, ProgramContext, Type};
use super::{MethodQualifier, FunctionSignature, Identifier};

#[parsable]
pub struct InterfaceMethodDeclaration {
    pub qualifier: Option<MethodQualifier>,
    pub name: Identifier,
    #[parsable(suffix=";")]
    pub signature: FunctionSignature
}

impl InterfaceMethodDeclaration {
    pub fn process(&self, context: &mut ProgramContext) -> (MethodQualifier, Identifier, Vec<(Identifier, Type)>, Option<Type>) {
        let (arguments, return_type) = self.signature.process(context);
        let name = self.name.clone();
        let method_qualifier = self.qualifier.unwrap_or(MethodQualifier::Regular);

        (method_qualifier, name, arguments, return_type)
    }
}