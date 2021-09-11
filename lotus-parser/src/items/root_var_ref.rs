use parsable::parsable;
use crate::program::{AccessType, ProgramContext, VariableKind, IrFragment};
use super::{VarRef, VarRefPrefix};

#[parsable]
pub struct RootVarRef {
    pub prefix: Option<VarRefPrefix>,
    pub var_ref: VarRef
}

impl RootVarRef {
    pub fn has_side_effects(&self) -> bool {
        self.var_ref.has_side_effects()
    }

    pub fn process(&self, access_type: AccessType, context: &mut ProgramContext) -> Option<IrFragment> {
        let mut result = None;

        match &self.prefix {
            Some(prefix) => {
                if let Some(prefix_wasm) = prefix.process(self, context) {
                    if let Some(wasm) = self.var_ref.process_as_field(&prefix_wasm.ty, access_type, context) {
                        result = Some(IrFragment::merge(wasm.ty.clone(), vec![prefix_wasm, wasm]));
                    }
                }
            },
            None => {
                result = self.var_ref.process_as_variable(access_type, context);
            }
        }

        result
    }
}