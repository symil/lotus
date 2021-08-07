use parsable::parsable;

use crate::program::{ProgramContext, Wasm};

use super::{VarRef, VarRefPrefix};

#[parsable]
pub struct RootVarRef {
    pub prefix: Option<VarRefPrefix>,
    pub var_ref: VarRef
}

impl RootVarRef {
    pub fn has_this_prefix(&self) -> bool {
        match self.prefix {
            Some(VarRefPrefix::This) => true,
            _ => false
        }
    }

    pub fn has_payload_prefix(&self) -> bool {
        match self.prefix {
            Some(VarRefPrefix::Payload) => true,
            _ => false
        }
    }

    pub fn process(&self, context: &mut ProgramContext) -> Option<Wasm> {
        match &self.prefix {
            Some(prefix) => {
                if let Some(prefix_wasm) = prefix.process(self, context) {
                    self.var_ref.process_as_field(&prefix_wasm.ty, context)
                } else {
                    None
                }
            },
            None => {
                if context.inside_const_expr {
                    todo!()
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
                } else {
                    self.var_ref.process_as_variable(context)
                    // if let Some(var_info) = context.get_var_info(&self.name) {
                    //     Some(Wasm::typed(
                    //         var_info.expr_type.clone(),
                    //         Wat::var_name(var_info.name.as_str())
                    //     ))
                    // } else {
                    //     None
                    // }
                }
            }
        }
    }
}