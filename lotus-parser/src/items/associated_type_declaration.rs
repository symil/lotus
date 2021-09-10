use parsable::parsable;
use crate::program::{ProgramContext, Type};
use super::{FullType, Identifier};

#[parsable]
pub struct AssociatedTypeDeclaration {
    #[parsable(prefix="type")]
    pub name: Identifier,
    #[parsable(prefix="=")]
    pub value: FullType
}

impl AssociatedTypeDeclaration {
    pub fn process(&self, context: &mut ProgramContext) -> (Identifier, Type) {
        (
            self.name.clone(),
            self.value.process(context).unwrap_or(Type::Void)
        )
    }
}