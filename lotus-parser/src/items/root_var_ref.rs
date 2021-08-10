use parsable::parsable;
use crate::program::{AccessType, ProgramContext, VariableScope, Wasm};
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
                    // if let Some(referenced_const) = self.const_declarations.get(&self.name) {
                    //     if let Some(_) = context.visit_constant(&self.name) {
                    //         context.error(&referenced_const.var_name, format!("circular reference to `{}`", &referenced_const.var_name));

                    //         None
                    //     } else {
                    //         self.get_expression_type(&referenced_const.init_value, context)
                    //     }
                    // } else {
                    //     context.error(&self.name, format!("undefined constant `{}`", &self.name));
                    //     None
                    // }
                
        }
    }
}