use parsable::parsable;
use crate::program::{AccessType, ProgramContext, VariableKind, Wasm};
use super::{VarRef, VarRefPrefix};

#[parsable]
pub struct RootVarRef {
    pub prefix: Option<VarRefPrefix>,
    pub var_ref: VarRef
}

impl RootVarRef {
    pub fn process(&self, access_type: AccessType, context: &mut ProgramContext) -> Option<Wasm> {
        match &self.prefix {
            Some(prefix) => {
                if let Some(prefix_wasm) = prefix.process(self, context) {
                    self.var_ref.process_as_field(&prefix_wasm.ty, access_type, context)
                } else {
                    None
                }
            },
            None => self.var_ref.process_as_variable(access_type, context)
        }
    }
}