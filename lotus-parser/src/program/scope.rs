use std::collections::HashMap;
use crate::items::Identifier;
use super::VariableInfo;

#[derive(Debug)]
pub struct Scope {
    pub kind: ScopeKind,
    pub variables: HashMap<Identifier, VariableInfo>,
}

#[derive(Debug)]
pub enum ScopeKind {
    Global,
    Function,
    Loop,
    Branch
}

impl Scope {
    pub fn new(kind: ScopeKind) -> Self {
        Self {
            kind,
            variables: HashMap::new()
        }
    }

    pub fn get_var_info(&self, var_name: &Identifier) -> Option<VariableInfo> {
        self.variables.get(var_name).cloned()
    }

    pub fn insert_var_info(&mut self, var_name: &Identifier, info: VariableInfo) {
        self.variables.insert(var_name.clone(), info);
    }
}