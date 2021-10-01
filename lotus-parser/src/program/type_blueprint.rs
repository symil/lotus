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
    pub self_type: Type,
    pub parent_type: Option<Type>,
    pub self_fields: IndexMap<String, Rc<FieldInfo>>,
    pub fields: IndexMap<String, Rc<FieldInfo>>,
    pub regular_methods: IndexMap<String, Link<FunctionBlueprint>>,
    pub static_methods: IndexMap<String, Link<FunctionBlueprint>>,
    pub dynamic_methods: Vec<Link<FunctionBlueprint>>,
    pub hook_event_callbacks: IndexMap<String, Vec<Link<FunctionBlueprint>>>,
    pub before_event_callbacks: IndexMap<String, Vec<Link<FunctionBlueprint>>>,
    pub after_event_callbacks: IndexMap<String, Vec<Link<FunctionBlueprint>>>,
}

#[derive(Debug, Clone)]
pub struct FieldInfo {
    pub owner: Link<TypeBlueprint>,
    pub name: Identifier,
    pub ty: Type,
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

    pub fn get_type(&self) -> Type {
        self.self_type.clone()
    }
}

impl GlobalItem for TypeBlueprint {
    fn get_name(&self) -> &Identifier { &self.name }
    fn get_visibility(&self) -> Visibility { self.visibility }
}