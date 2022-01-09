use parsable::parsable;
use super::{ParsedExpression, ParsedType, Identifier};

#[parsable]
pub struct ParsedFieldDeclaration {
    pub name: Identifier,
    #[parsable(prefix=":")]
    pub ty: Option<ParsedType>,
    #[parsable(prefix="=")]
    pub default_value: Option<ParsedExpression>,
    #[parsable(prefix=",")]
    pub comma: Option<()>
}