use std::{collections::hash_map::DefaultHasher, hash::{Hash, Hasher}, rc::Rc};
use indexmap::{IndexMap, IndexSet};
use parsable::DataLocation;
use crate::{items::{Identifier, StackType, TypeQualifier, Visibility}, utils::Link};
use super::{ActualTypeInfo, FunctionBlueprint, GenericTypeInfo, GlobalItem, InterfaceBlueprint, ProgramContext, ResolvedType, Type, TypeInstance};

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
    pub methods: IndexMap<String, Link<FunctionBlueprint>>,
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
            StackType::Pointer => Some("i32"),
        }
    }

    pub fn get_instance_id(&self, parameters: &[Rc<TypeInstance>]) -> u64 {
        let mut hasher = DefaultHasher::new();

        self.type_id.hash(&mut hasher);
        parameters.hash(&mut hasher);

        hasher.finish()
    }

    pub fn resolve(&self, parameters: &[Rc<TypeInstance>], context: &mut ProgramContext) -> Rc<TypeInstance> {
        let id = self.get_instance_id(parameters);

        if let Some(type_instance) = context.type_instances.get(&id) {
            return Rc::clone(type_instance);
        }

        let mut instance_generic_types = IndexMap::new();

        for (generic_type, parameter_type) in self.parameters.values().zip(parameters.iter()) {
            instance_generic_types.insert(generic_type.get_id(), Rc::clone(parameter_type));
        }

        for associated_type in self.associated_types.values() {
            
        }

        todo!()
    }
}

impl Link<TypeBlueprint> {
    pub fn get_info(&self) -> ActualTypeInfo {
        ActualTypeInfo {
            type_wrapped: self.clone(),
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