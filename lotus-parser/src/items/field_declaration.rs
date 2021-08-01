use parsable::parsable;

use super::{Identifier, Type};

#[parsable]
pub struct FieldDeclaration {
    pub name: Identifier,
    #[parsable(prefix=":")]
    pub type_: Type,
    // TODO: default value
}