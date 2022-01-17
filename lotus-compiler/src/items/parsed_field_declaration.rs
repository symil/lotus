use parsable::parsable;
use super::{ParsedExpression, ParsedType, Identifier, ParsedColon, ParsedEqual, ParsedComma};

#[parsable(none_cascade = true)]
pub struct ParsedFieldDeclaration {
    pub name: Identifier,
    pub colon: Option<ParsedColon>,
    pub ty: Option<ParsedType>,
    pub equal: Option<ParsedEqual>,
    pub default_value: Option<ParsedExpression>,
    #[parsable(cascade = false)]
    pub comma: Option<ParsedComma>,
}

impl ParsedFieldDeclaration {
    
}