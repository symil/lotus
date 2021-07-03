use lotus_parsable::parsable;

use super::identifier::Identifier;

#[parsable]
#[derive(Debug)]
pub struct FieldDeclaration {
    pub name: Identifier,
    #[parsable(prefix=":")]
    pub ty: Option<Identifier>
}