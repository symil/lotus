use parsable::parsable;
use crate::program::{ProgramContext, Type};
use super::{FunctionQualifier, FunctionSignature, Identifier};

#[parsable]
pub struct InterfaceMethodDeclaration {
    pub qualifier: Option<FunctionQualifier>,
    pub name: Identifier,
    #[parsable(suffix=";")]
    pub signature: FunctionSignature
}

impl InterfaceMethodDeclaration {
    pub fn process(&self, context: &mut ProgramContext) -> (bool, Identifier, Vec<(Identifier, Type)>, Option<Type>) {
        let (arguments, return_type) = self.signature.process(context);
        let name = self.name.clone();
        let is_static = self.qualifier.contains(&FunctionQualifier::Static);

        (is_static, name, arguments, return_type)
    }
}