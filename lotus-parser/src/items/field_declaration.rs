use parsable::parsable;
use super::{Expression, FullType, Identifier};

#[parsable]
pub struct FieldDeclaration {
    pub name: Identifier,
    #[parsable(prefix=":")]
    pub ty: Option<FullType>,
    #[parsable(prefix="=")]
    pub default_value: Option<Expression>,
    #[parsable(prefix=",")]
    pub comma: Option<()>
}