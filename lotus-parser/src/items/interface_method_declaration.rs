use parsable::parsable;
use crate::program::{FieldKind, ProgramContext, Type};
use super::{FunctionQualifier, FunctionSignature, Identifier};

#[parsable]
pub struct InterfaceMethodDeclaration {
    pub qualifier: Option<FunctionQualifier>,
    pub name: Identifier,
    #[parsable(suffix=";")]
    pub signature: FunctionSignature
}

impl InterfaceMethodDeclaration {
    pub fn process(&self, context: &mut ProgramContext) -> (FieldKind, Identifier, Vec<(Identifier, Type)>, Option<Type>) {
        let (arguments, return_type) = self.signature.process(context);
        let name = self.name.clone();
        let method_kind = match &self.qualifier {
            Some(FunctionQualifier::Static) => FieldKind::Static,
            _ => FieldKind::Regular,
        };

        (method_kind, name, arguments, return_type)
    }
}