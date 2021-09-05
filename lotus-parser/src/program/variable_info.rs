use crate::generation::Wat;

use super::TypeOld;

#[derive(Debug, Clone)]
pub struct VariableInfo {
    pub wasm_name: String,
    pub ty: TypeOld,
    pub kind: VariableKind
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum VariableKind {
    Global,
    Local,
    Argument
}

impl VariableInfo {
    pub fn new(wasm_name: String, ty: TypeOld, kind: VariableKind) -> Self {
        Self { wasm_name, ty, kind }
    }

    pub fn get_to_stack(&self) -> Wat {
        match &self.kind {
            VariableKind::Global => Wat::get_global(&self.wasm_name),
            VariableKind::Local => Wat::get_local(&self.wasm_name),
            VariableKind::Argument => Wat::get_local(&self.wasm_name),
        }
    }

    pub fn set_from_stack(&self) -> Wat {
        match &self.kind {
            VariableKind::Global => Wat::set_global_from_stack(&self.wasm_name),
            VariableKind::Local => Wat::set_local_from_stack(&self.wasm_name),
            VariableKind::Argument => Wat::set_local_from_stack(&self.wasm_name),
        }
    }

    pub fn get_wasm_name(&self) -> &str {
        self.wasm_name.as_str()
    }
}

impl Default for VariableInfo {
    fn default() -> Self {
        Self::new(String::new(), TypeOld::Void, VariableKind::Local)
    }
}