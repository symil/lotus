use parsable::parsable;

use super::{Identifier, AnyType};

#[parsable]
pub struct FieldDeclaration {
    pub name: Identifier,
    #[parsable(prefix=":")]
    pub ty: AnyType,
    // TODO: default value
}