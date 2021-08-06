use parsable::parsable;

use super::{Expression, Identifier};

#[parsable]
pub struct ObjectFieldInitialization {
    pub name: Identifier,
    #[parsable(prefix=":")]
    pub value: Expression
}