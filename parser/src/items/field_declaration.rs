use crate::located_data::LocatedData;
use super::identifier::Identifier;

item! {
    struct FieldDeclaration {
        name: LocatedData<Identifier>,
        ty: LocatedData<Identifier>
    }

    @entry => FieldDeclaration {
        name: parse!(entry),
        ty: parse!(entry)
    }
}