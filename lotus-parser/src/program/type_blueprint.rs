use indexmap::{IndexMap, IndexSet};
use parsable::DataLocation;
use crate::{items::{Identifier, StackType, TypeQualifier, Visibility}, utils::Link};
use super::{ActualTypeInfo, FunctionBlueprint, GlobalItem, InterfaceBlueprint, Type};

#[derive(Debug, Default)]
pub struct TypeBlueprint {
    pub type_id: u64,
    pub name: Identifier,
    pub visibility: Visibility,
    pub qualifier: TypeQualifier,
    pub stack_type: StackType,
    pub parameters: IndexMap<String, Link<ParameterType>>,
    pub associated_types: IndexMap<String, AssociatedType>,
    pub parent: Option<ActualTypeInfo>,
    pub inheritance_chain: Vec<ActualTypeInfo>, // from the most "parent" type to the most "child", including self
    pub fields: IndexMap<String, FieldDetails>,
    pub static_fields: IndexMap<String, FieldDetails>,
    pub methods: IndexMap<String, Link<FunctionBlueprint>>,
    pub static_methods: IndexMap<String, Link<FunctionBlueprint>>,
    pub dynamic_methods: Vec<Link<FunctionBlueprint>>,
    pub hook_event_callbacks: IndexMap<String, Vec<Link<FunctionBlueprint>>>,
    pub before_event_callbacks: IndexMap<String, Vec<Link<FunctionBlueprint>>>,
    pub after_event_callbacks: IndexMap<String, Vec<Link<FunctionBlueprint>>>,
}

#[derive(Debug, Clone)]
pub struct ParameterType {
    pub name: Identifier,
    pub required_interfaces: Vec<Link<InterfaceBlueprint>>
}

#[derive(Debug, Clone)]
pub struct AssociatedType {
    pub owner: Link<TypeBlueprint>,
    pub name: Identifier,
    pub value: Type
}

#[derive(Debug, Clone)]
pub struct FieldDetails {
    pub owner: Link<TypeBlueprint>,
    pub name: Identifier,
    pub ty: Type,
    pub offset: usize,
}

impl TypeBlueprint {
    pub fn get_wasm_type(&self) -> Option<&'static str> {
        match self.stack_type {
            StackType::Void => None,
            StackType::Int => Some("i32"),
            StackType::Float => Some("f32"),
            StackType::Pointer => Some("i32"),
        }
    }
}

impl Link<TypeBlueprint> {
    pub fn get_info(&self) -> ActualTypeInfo {
        ActualTypeInfo {
            type_blueprint: self.clone(),
            parameters: self.borrow().parameters.values().map(|param| Type::Parameter(param.clone())).collect(),
        }
    }
}

impl GlobalItem for TypeBlueprint {
    fn get_name(&self) -> &Identifier { &self.name }
    fn get_visibility(&self) -> Visibility { self.visibility }
}

impl Default for StackType {
    fn default() -> Self {
        Self::Void
    }
}