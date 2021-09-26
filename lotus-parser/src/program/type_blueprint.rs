use std::{collections::hash_map::DefaultHasher, hash::{Hash, Hasher}, rc::Rc};
use indexmap::{IndexMap, IndexSet};
use parsable::DataLocation;
use crate::{items::{Identifier, StackType, TypeQualifier, Visibility}, utils::Link};
use super::{ActualTypeInfo, FunctionBlueprint, GenericTypeInfo, GlobalItem, InterfaceBlueprint, LOAD_FUNC_NAME, ProgramContext, ResolvedType, STORE_FUNC_NAME, Type, TypeInstanceContent};

#[derive(Debug)]
pub struct TypeBlueprint {
    pub type_id: u64,
    pub name: Identifier,
    pub visibility: Visibility,
    pub qualifier: TypeQualifier,
    pub stack_type: StackType,
    pub parameters: IndexMap<String, Rc<GenericTypeInfo>>,
    pub associated_types: IndexMap<String, Type>,
    pub parent: Option<ActualTypeInfo>,
    pub inheritance_chain: Vec<ActualTypeInfo>, // from the most "parent" type to the most "child", including self
    pub fields: IndexMap<String, Rc<FieldDetails>>,
    pub static_fields: IndexMap<String, Rc<FieldDetails>>,
    pub regular_methods: IndexMap<String, Link<FunctionBlueprint>>,
    pub static_methods: IndexMap<String, Link<FunctionBlueprint>>,
    pub dynamic_methods: Vec<Link<FunctionBlueprint>>,
    pub hook_event_callbacks: IndexMap<String, Vec<Link<FunctionBlueprint>>>,
    pub before_event_callbacks: IndexMap<String, Vec<Link<FunctionBlueprint>>>,
    pub after_event_callbacks: IndexMap<String, Vec<Link<FunctionBlueprint>>>,
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
        }
    }

    pub fn generate_builtin_methods(&mut self) {
        let methods = &[
            (STORE_FUNC_NAME, "store"),
            (LOAD_FUNC_NAME, "load")
        ];
    }
}

impl Link<TypeBlueprint> {
    pub fn get_info(&self) -> ActualTypeInfo {
        ActualTypeInfo {
            type_blueprint: self.clone(),
            parameters: self.borrow().parameters.values().map(|info| Type::TypeParameter(Rc::clone(info))).collect(),
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