use parsable::parsable;
use crate::program::{FieldKind, MethodQualifier};

#[parsable]
pub struct ParsedMethodQualifier {
    pub token: ParsedMethodQualifierToken
}

#[parsable]
#[derive(Clone, Copy, PartialEq)]
pub enum ParsedMethodQualifierToken {
    Static = "static",
    Dyn = "dyn"
}

impl ParsedMethodQualifier {
    pub fn process(&self) -> MethodQualifier {
        match &self.token {
            ParsedMethodQualifierToken::Static => MethodQualifier::Static,
            ParsedMethodQualifierToken::Dyn => MethodQualifier::Dynamic,
        }
    }
}