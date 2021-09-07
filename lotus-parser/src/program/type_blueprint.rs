use indexmap::{IndexMap, IndexSet};
use parsable::DataLocation;
use crate::items::{Identifier, StackType, TypeQualifier, Visibility};
use super::{GlobalItem, Type, TypeRef};

#[derive(Debug, Default)]
pub struct TypeBlueprint {
    pub type_id: u64,
    pub name: String,
    pub location: DataLocation,
    pub visibility: Visibility,
    pub qualifier: TypeQualifier,
    pub stack_type: StackType,
    pub generics: IndexSet<String>,
    pub parent: Option<TypeRef>,
    pub inheritance_chain: Vec<TypeRef>, // from the most "parent" type to the most "child", including self
    pub fields: IndexMap<String, FieldDetails>,
    pub static_fields: IndexMap<String, FieldDetails>,
    pub methods: IndexMap<String, MethodDetails>,
    pub static_methods: IndexMap<String, MethodDetails>,
    pub hook_event_callbacks: IndexMap<String, Vec<MethodDetails>>,
    pub before_event_callbacks: IndexMap<String, Vec<MethodDetails>>,
    pub after_event_callbacks: IndexMap<String, Vec<MethodDetails>>,
}

#[derive(Debug)]
pub struct FieldDetails {
    pub name: Identifier,
    pub ty: Type,
    pub owner_type_id: u64,
    pub offset: usize
}

#[derive(Debug)]
pub struct MethodDetails {
    pub function_id: u64,
    pub is_from_self: usize
}

impl TypeBlueprint {
    pub fn get_typeref(&self) -> TypeRef {
        TypeRef {
            type_id: self.type_id,
            type_context: Some(self.type_id),
            generic_values: self.generics.iter().map(|name| Type::generic(name.to_string(), self.type_id)).collect(),
        }
    }
}

impl GlobalItem for TypeBlueprint {
    fn get_id(&self) -> u64 { self.type_id }
    fn get_name(&self) -> &str { &self.name }
    fn get_location(&self) -> &DataLocation { &self.location }
    fn get_visibility(&self) -> Visibility { self.visibility }
}

impl Default for StackType {
    fn default() -> Self {
        Self::Void
    }
}