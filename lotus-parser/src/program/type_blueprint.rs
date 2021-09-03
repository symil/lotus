use indexmap::{IndexMap, IndexSet};
use parsable::DataLocation;
use super::{GlobalItem, ValueType, ItemVisibility};

#[derive(Debug)]
pub struct TypeBlueprint {
    pub id: u64,
    pub name: String,
    pub location: DataLocation,
    pub visibility: ItemVisibility,
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

#[derive(Debug, PartialEq)]
pub enum StackType {
    Void,
    Int,
    Float,
    Pointer,
}

#[derive(Debug)]
pub struct FieldDetails {
    pub ty: ValueType,
    pub is_from_self: bool,
    pub offset: usize
}

#[derive(Debug)]
pub struct MethodDetails {
    pub function_id: u64,
    pub is_from_self: usize
}

impl GlobalItem for TypeBlueprint {
    fn get_id(&self) -> u64 { self.id }
    fn get_name(&self) -> &str { &self.name }
    fn get_location(&self) -> &DataLocation { &self.location }
    fn get_visibility(&self) -> ItemVisibility { self.visibility }
}