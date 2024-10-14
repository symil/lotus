use parsable::parsable;
use crate::program::MethodMetaQualifier;

#[parsable]
pub struct ParsedMethodMetaQualifier {
    pub token: ParsedMethodMetaQualifierToken
}

#[parsable]
#[derive(Clone, Copy, PartialEq)]
pub enum ParsedMethodMetaQualifierToken {
    Autogen = "autogen"
}

impl ParsedMethodMetaQualifier {
    pub fn process(&self) -> MethodMetaQualifier {
        match &self.token {
            ParsedMethodMetaQualifierToken::Autogen => MethodMetaQualifier::Autogen,
        }
    }
}