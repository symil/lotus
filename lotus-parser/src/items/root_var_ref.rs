use parsable::parsable;
use crate::program::{AccessType, ProgramContext, VariableKind, Vasm};
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

    pub fn process(&self, access_type: AccessType, context: &mut ProgramContext) -> Option<Vasm> {
        let mut result = None;

        match &self.prefix {
            Some(prefix) => {
                if let Some(prefix_vasm) = prefix.process(self, context) {
                    if let Some(field_vasm) = self.var_ref.process_as_field(&prefix_vasm.ty, access_type, context) {
                        result = Some(Vasm::merge(field_vasm.ty.clone(), vec![prefix_vasm, field_vasm]));
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