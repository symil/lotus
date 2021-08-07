use parsable::parsable;

use super::{Identifier, FullType};

#[parsable]
pub struct FieldDeclaration {
    pub name: Identifier,
    #[parsable(prefix=":")]
    pub ty: FullType,
    // TODO: default value
}