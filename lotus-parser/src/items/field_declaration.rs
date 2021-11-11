use parsable::parsable;
use super::{Expression, ParsedType, Identifier};

#[parsable]
pub struct FieldDeclaration {
    pub name: Identifier,
    #[parsable(prefix=":")]
    pub ty: Option<ParsedType>,
    #[parsable(prefix="=")]
    pub default_value: Option<Expression>,
    #[parsable(prefix=",")]
    pub comma: Option<()>
}