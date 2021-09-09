use indexmap::{IndexMap, IndexSet};
use parsable::DataLocation;
use crate::items::{Identifier, StackType, TypeQualifier, Visibility};
use super::{GlobalItem, Type, ActualTypeInfo};

#[derive(Debug, Default)]
pub struct TypeBlueprint {
    pub type_id: u64,
    pub name: String,
    pub location: DataLocation,
    pub visibility: Visibility,
    pub qualifier: TypeQualifier,
    pub stack_type: StackType,
    pub parameters: IndexMap<String, TypeParameter>,
    pub parent: Option<ActualTypeInfo>,
    pub inheritance_chain: Vec<ActualTypeInfo>, // from the most "parent" type to the most "child", including self
    pub fields: IndexMap<String, FieldDetails>,
    pub static_fields: IndexMap<String, FieldDetails>,
    pub methods: IndexMap<String, MethodDetails>,
    pub static_methods: IndexMap<String, MethodDetails>,
    pub hook_event_callbacks: IndexMap<String, Vec<MethodDetails>>,
    pub before_event_callbacks: IndexMap<String, Vec<MethodDetails>>,
    pub after_event_callbacks: IndexMap<String, Vec<MethodDetails>>,
}

#[derive(Debug)]
pub struct TypeParameter {
    pub name: Identifier,
    pub required_interfaces: Vec<u64>
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
    pub owner_type_id: u64,
}

impl TypeBlueprint {
    pub fn get_typeref(&self) -> ActualTypeInfo {
        ActualTypeInfo {
            type_id: self.type_id,
            name: self.name.clone(),
            parameters: self.parameters.values().map(|param| Type::generic(param.name.to_string(), self.type_id)).collect(),
        }
    }

    pub fn get_wasm_type(&self) -> Option<&'static str> {
        match self.stack_type {
            StackType::Void => None,
            StackType::Int => Some("i32"),
            StackType::Float => Some("f32"),
            StackType::Pointer => Some("i32"),
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