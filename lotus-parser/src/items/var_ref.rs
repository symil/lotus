use parsable::parsable;

use crate::{generation::Wat, program::{ProgramContext, Type, Wasm, process_system_variable}};

use super::{Identifier, VarRefPrefix, process_field_access};

#[parsable]
pub struct VarRef {
    pub prefix: Option<VarRefPrefix>,
    pub name: Identifier
}

impl VarRef {
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
                let prefix_var_opt = match prefix {
                    VarRefPrefix::This => {
                        if context.get_this_type().is_none() {
                            context.error(prefix, "no `this` value can be referenced in this context");
                        }

                        context.get_this_type()
                    },
                    VarRefPrefix::Payload => {
                        if context.get_payload_type().is_none() {
                            context.error(prefix, "no `payload` value can be referenced in this context");
                        }

                        context.get_payload_type()
                    },
                    VarRefPrefix::System => {
                        let result = process_system_variable(&self.name, context);

                        if result.is_none() {
                            context.error(prefix, format!("undefined system variable `@{}`", &self.name));
                        }

                        return result;
                    }
                };

                if let Some(prefix_var) = &prefix_var_opt {
                    process_field_access(&prefix_var.expr_type, &self.name, context)
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
                    if let Some(var_info) = context.get_var_info(&self.name) {
                        Some(Wasm::typed(
                            var_info.expr_type.clone(),
                            Wat::var_name(var_info.name.as_str())
                        ))
                    } else {
                        None
                    }
                }
            }
        }
    }
}