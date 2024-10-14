use parsable::parsable;
use crate::program::ProgramContext;
use super::Identifier;

#[parsable]
pub struct ParsedInterfaceAssociatedTypeDeclaration {
    #[parsable(prefix="type", suffix=";")]
    pub name: Identifier
}

impl ParsedInterfaceAssociatedTypeDeclaration {
    pub fn process(&self, context: &mut ProgramContext) -> Identifier {
        self.name.clone()
    }
}