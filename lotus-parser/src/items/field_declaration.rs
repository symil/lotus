use parsable::parsable;

use super::identifier::Identifier;

#[parsable(located)]
#[derive(Debug)]
pub struct FieldDeclaration {
    pub name: Identifier,
    #[parsable(prefix=":")]
    pub ty: Option<Identifier>
}