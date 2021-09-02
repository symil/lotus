use indexmap::{IndexMap, IndexSet};
use parsable::DataLocation;
use crate::items::Identifier;

use super::TypeInstance;

pub struct TypeBlueprint {
    pub id: u64,
    pub name: String,
    pub location: DataLocation,
    pub generics: IndexSet<String>,
    pub fields: IndexMap<String, FieldDetails>
}

pub struct FieldDetails {
    pub ty: TypeInstance
}