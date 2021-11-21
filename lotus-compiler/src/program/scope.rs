use std::{collections::HashMap, rc::Rc};
use crate::items::Identifier;
use super::{VariableInfo, insert_in_vec_hashmap};

#[derive(Debug)]
pub struct Scope {
    pub kind: ScopeKind,
    pub variables: HashMap<String, Vec<VariableInfo>>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScopeKind {
    Global,
    Function,
    Loop,
    Branch,
    Local
}

impl Scope {
    pub fn new(kind: ScopeKind) -> Self {
        Self {
            kind,
            variables: HashMap::new()
        }
    }

    pub fn get_var_info(&self, var_name: &str) -> Option<&VariableInfo> {
        self.variables.get(var_name)?.last()
    }

    pub fn insert_var_info(&mut self, info: &VariableInfo) {
        insert_in_vec_hashmap(&mut self.variables, &info.name().to_string(), info.clone());
    }
}

impl ScopeKind {
    pub fn get_depth(&self) -> u32 {
        match self {
            ScopeKind::Global => 0,
            ScopeKind::Function => 0,
            ScopeKind::Loop => 2,
            ScopeKind::Branch => 2,
            ScopeKind::Local => 2,
        }
    }

    pub fn is_function(&self) -> bool {
        match self {
            ScopeKind::Function => true,
            _ => false
        }
    }
}