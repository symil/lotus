use super::identifier::Identifier;
use super::type_qualifier::TypeQualifier;
use super::field_declaration::FieldDeclaration;

pub struct TypeDeclaration {
    pub qualifier: TypeQualifier,
    pub name: Identifier,
    // #[parsable(min=1, sep=",", prefix="extends")]
    pub extends: Vec<Identifier>,
    // #[parsable(prefix="{", suffix="}")]
    pub fields: Vec<FieldDeclaration>
}