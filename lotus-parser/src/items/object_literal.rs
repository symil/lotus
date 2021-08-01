use parsable::parsable;

use super::{Expression, Identifier};

#[parsable]
pub struct ObjectLiteral {
    pub type_name: Identifier,
    #[parsable(brackets="{}", separator=",")]
    pub fields: Vec<FieldInitialization>
}

#[parsable]
pub struct FieldInitialization {
    pub name: Identifier,
    pub value: Expression
}