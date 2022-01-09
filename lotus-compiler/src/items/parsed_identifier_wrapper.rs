use std::borrow::Cow;
use parsable::parsable;
use crate::program::ProgramContext;
use super::{Identifier, ParsedMacroIdentifier};

#[parsable]
pub enum ParsedIdentifierWrapper {
    Identifier(Identifier),
    Macro(ParsedMacroIdentifier)
}

impl ParsedIdentifierWrapper {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Cow<Identifier>> {
        match self {
            ParsedIdentifierWrapper::Identifier(identifier) => Some(Cow::Borrowed(identifier)),
            ParsedIdentifierWrapper::Macro(mac) => mac.process(context).map(|identifier| Cow::Owned(identifier)),
        }
    }
}