use crate::generation::Wat;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum VariableScope {
    Global,
    Local,
    Argument
}

impl VariableScope {
    pub fn get_to_stack(&self, var_name: &str) -> Wat {
        match self {
            VariableScope::Global => Wat::get_global(var_name),
            VariableScope::Local => Wat::get_local(var_name),
            VariableScope::Argument => Wat::get_local(var_name),
        }
    }

    pub fn set_from_stack(&self, var_name: &str) -> Wat {
        match self {
            VariableScope::Global => Wat::set_global_from_stack(var_name),
            VariableScope::Local => Wat::set_local_from_stack(var_name),
            VariableScope::Argument => Wat::set_local_from_stack(var_name),
        }
    }
}

impl Default for VariableScope {
    fn default() -> Self {
        Self::Global
    }
}