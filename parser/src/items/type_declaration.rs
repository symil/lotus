use super::identifier::Identifier;
use super::type_qualifier::TypeQualifier;
use super::field_declaration::FieldDeclaration;

struct TypeDeclaration2 {
    qualifier: TypeQualifier,
    name: Identifier,
    #[parsable(min=1, sep=",", prefix="extends")]
    extends: Vec<Identifier>,
    #[parsable(prefix="{", suffix="}")]
    fields: Vec<FieldDeclaration>
}

item! {
    struct TypeDeclaration {
        qualifier: TypeQualifier,
        name: Identifier,
        extends: Vec<Identifier>,
        fields: Vec<FieldDeclaration>
    }

    @entry => TypeDeclaration {
        qualifier: parse!(entry),
        name: parse!(entry),
        extends: parse_list!(entry, identifier),
        fields: vec![]
    }
}