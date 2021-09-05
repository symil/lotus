use indexmap::{IndexMap, IndexSet};
use parsable::DataLocation;
use crate::items::{StackType, Visibility};
use super::{GlobalItem, Type};

#[derive(Debug, Default)]
pub struct TypeBlueprint {
    pub id: u64,
    pub name: String,
    pub location: DataLocation,
    pub visibility: Visibility,
    pub stack_type: StackType,
    pub generics: IndexSet<String>,
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
    pub ty: Type,
    pub is_from_self: bool,
    pub offset: usize
}

#[derive(Debug)]
pub struct MethodDetails {
    pub function_id: u64,
    pub is_from_self: usize
}

impl TypeBlueprint {
    pub fn is_class(&self) -> bool {
        self.stack_type == StackType::Pointer
    }
}

impl GlobalItem for TypeBlueprint {
    fn get_id(&self) -> u64 { self.id }
    fn get_name(&self) -> &str { &self.name }
    fn get_location(&self) -> &DataLocation { &self.location }
    fn get_visibility(&self) -> Visibility { self.visibility }
}

impl Default for StackType {
    fn default() -> Self {
        Self::Void
    }
}