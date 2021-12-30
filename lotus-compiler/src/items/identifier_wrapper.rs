use std::borrow::Cow;

use parsable::parsable;
use crate::program::ProgramContext;
use super::{Identifier, MacroIdentifier};

#[parsable]
pub enum IdentifierWrapper {
    Identifier(Identifier),
    Macro(MacroIdentifier)
}

impl IdentifierWrapper {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Cow<Identifier>> {
        match self {
            IdentifierWrapper::Identifier(identifier) => Some(Cow::Borrowed(identifier)),
            IdentifierWrapper::Macro(mac) => mac.process(context).map(|identifier| Cow::Owned(identifier)),
        }
    }
}