use crate::located_data::{LocatedData};
use super::identifier::Identifier;
use super::type_qualifier::TypeQualifier;

item! {
    struct TypeDeclaration {
        qualifier: LocatedData<TypeQualifier>,
        name: LocatedData<Identifier>,
        extends: Vec<LocatedData<Identifier>>,
        fields: Vec<LocatedData<FieldDeclaration>>
    }

    entry => {
        let mut iterator = iterator!(entry);

        TypeDeclaration {
            qualifier: parse!(iterator),
            name: parse!(iterator),
            extends: parse_list!(iterator, identifier),
            fields: vec![]
        }
}
}

#[derive(Debug)]
pub struct FieldDeclaration {
    pub name: LocatedData<Identifier>,
    pub ty: LocatedData<Identifier>,

}