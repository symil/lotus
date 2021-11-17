use parsable::parsable;
use crate::program::ProgramContext;
use super::{Identifier, Macro};

#[parsable]
pub enum IdentifierWrapper {
    Identifier(Identifier),
    Macro(Macro)
}

impl IdentifierWrapper {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Identifier> {
        match self {
            IdentifierWrapper::Identifier(identifier) => Some(identifier.clone()),
            IdentifierWrapper::Macro(mac) => mac.process_as_name(context),
        }
    }
}