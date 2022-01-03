use parsable::parsable;
use crate::program::{ProgramContext, Type};
use super::{ParsedType, Identifier};

#[parsable]
pub struct AssociatedTypeDeclaration {
    #[parsable(prefix="type")]
    pub name: Identifier,
    #[parsable(prefix="=", suffix=";")]
    pub value: ParsedType
}

impl AssociatedTypeDeclaration {
    pub fn process(&self, context: &mut ProgramContext) -> (Identifier, Type) {
        (
            self.name.clone(),
            self.value.process(false, context).unwrap_or(Type::undefined())
        )
    }
}