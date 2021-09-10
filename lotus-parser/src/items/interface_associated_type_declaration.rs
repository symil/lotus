use parsable::parsable;
use crate::program::ProgramContext;
use super::Identifier;

#[parsable]
pub struct InterfaceAssociatedTypeDeclaration {
    #[parsable(suffix=";")]
    pub name: Identifier
}

impl InterfaceAssociatedTypeDeclaration {
    pub fn process(&self, context: &mut ProgramContext) -> Identifier {
        self.name.clone()
    }
}