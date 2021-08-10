use crate::generation::Wat;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum VariableScope {
    Local,
    Global
}

impl VariableScope {
    pub fn get_to_stack(&self, var_name: &str) -> Wat {
        match self {
            VariableScope::Local => Wat::get_local(var_name),
            VariableScope::Global => Wat::get_global(var_name),
        }
    }

    pub fn set_from_stack(&self, var_name: &str) -> Wat {
        match self {
            VariableScope::Local => Wat::set_local_from_stack(var_name),
            VariableScope::Global => Wat::set_global_from_stack(var_name),
        }
    }
}

impl Default for VariableScope {
    fn default() -> Self {
        Self::Global
    }
}