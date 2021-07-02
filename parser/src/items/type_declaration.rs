use crate::located_data::{LocatedData};
use super::identifier::Identifier;
use super::type_qualifier::TypeQualifier;
use super::field_declaration::FieldDeclaration;

item! {
    struct TypeDeclaration {
        qualifier: LocatedData<TypeQualifier>,
        name: LocatedData<Identifier>,
        extends: Vec<LocatedData<Identifier>>,
        fields: Vec<LocatedData<FieldDeclaration>>
    }

    @entry => TypeDeclaration {
        qualifier: parse!(entry),
        name: parse!(entry),
        extends: parse_list!(entry, identifier),
        fields: vec![]
    }
}