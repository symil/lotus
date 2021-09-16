use std::{collections::HashMap, hash::Hash, rc::Rc};
use indexmap::IndexMap;
use crate::utils::Link;
use super::{FunctionInstance, TypeBlueprint};

#[derive(Debug, Clone)]
pub struct TypeInstance {
    pub id: u64,
    pub blueprint: Link<TypeBlueprint>,
    pub stack_type: Option<&'static str>,
    pub generic_types: IndexMap<usize, Rc<TypeInstance>>,
    pub methods: IndexMap<String, Rc<FunctionInstance>>,
    pub static_methods: IndexMap<String, Rc<FunctionInstance>>,
    pub dynamic_methods: Vec<Rc<FunctionInstance>>,
    pub hook_event_callbacks: IndexMap<String, Vec<Rc<FunctionInstance>>>,
    pub before_event_callbacks: IndexMap<String, Vec<Rc<FunctionInstance>>>,
    pub after_event_callbacks: IndexMap<String, Vec<Rc<FunctionInstance>>>,
}

impl Hash for TypeInstance {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}