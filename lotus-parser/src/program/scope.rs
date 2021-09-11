use std::{collections::HashMap, rc::Rc};
use crate::items::Identifier;
use super::VariableInfo;

#[derive(Debug)]
pub struct Scope {
    pub kind: ScopeKind,
    pub depth: u32,
    pub variables: HashMap<String, Rc<VariableInfo>>,
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
    pub fn new(kind: ScopeKind, depth: u32) -> Self {
        Self {
            kind,
            depth,
            variables: HashMap::new()
        }
    }

    pub fn get_var_info(&self, var_name: &Identifier) -> Option<&Rc<VariableInfo>> {
        self.variables.get(var_name.as_str())
    }

    pub fn insert_var_info(&mut self, info: &Rc<VariableInfo>) {
        self.variables.insert(info.name.to_string(), Rc::clone(info));
    }
}

impl ScopeKind {
    pub fn get_depth(&self) -> u32 {
        match self {
            ScopeKind::Global => 0,
            ScopeKind::Function => 1,
            ScopeKind::Loop => 2,
            ScopeKind::Branch => 2,
            ScopeKind::Local => 1
        }
    }
}