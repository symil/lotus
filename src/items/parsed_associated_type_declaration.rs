use parsable::parsable;
use crate::program::{ProgramContext, Type};
use super::{ParsedType, Identifier};

#[parsable]
pub struct ParsedAssociatedTypeDeclaration {
    #[parsable(prefix="type")]
    pub name: Identifier,
    #[parsable(prefix="=", suffix=";")]
    pub value: ParsedType
}

impl ParsedAssociatedTypeDeclaration {
    pub fn process(&self, context: &mut ProgramContext) -> (Identifier, Type) {
        (
            self.name.clone(),
            self.value.process(false, None, context).unwrap_or(Type::undefined())
        )
    }
}