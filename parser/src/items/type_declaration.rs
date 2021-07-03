use lotus_parsable::parsable;

use super::identifier::Identifier;
use super::type_qualifier::TypeQualifier;
use super::field_declaration::FieldDeclaration;

#[parsable]
#[derive(Debug)]
pub struct TypeDeclaration {
    pub qualifier: TypeQualifier,
    pub name: Identifier,
    #[parsable(optional=true, prefix="extends", sep=",", min=1)]
    pub extends: Vec<Identifier>,
    #[parsable(brackets="{}", sep=",")]
    pub fields: Vec<FieldDeclaration>
}