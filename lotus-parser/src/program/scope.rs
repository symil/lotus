use std::collections::HashMap;
use crate::items::Identifier;
use super::VariableInfo;

#[derive(Debug)]
pub struct Scope {
    pub kind: ScopeKind,
    pub depth: i32,
    pub variables: HashMap<Identifier, VariableInfo>,
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
    pub fn new(kind: ScopeKind, depth: i32) -> Self {
        Self {
            kind,
            depth,
            variables: HashMap::new()
        }
    }

    pub fn get_var_info(&self, var_name: &Identifier) -> Option<VariableInfo> {
        self.variables.get(var_name).cloned()
    }

    pub fn insert_var_info(&mut self, var_name: Identifier, info: VariableInfo) {
        self.variables.insert(var_name, info);
    }
}

impl ScopeKind {
    pub fn get_depth(&self) -> i32 {
        match self {
            ScopeKind::Global => 0,
            ScopeKind::Function => 1,
            ScopeKind::Loop => 2,
            ScopeKind::Branch => 2,
            ScopeKind::Local => 1
        }
    }
}