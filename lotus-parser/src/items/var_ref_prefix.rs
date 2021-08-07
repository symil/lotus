use std::ptr::NonNull;

use parsable::{DataLocation, Parsable, parsable};

use crate::{generation::Wat, program::{ProgramContext, Type, Wasm}};

#[parsable(impl_display=true)]
#[derive(PartialEq, Copy)]
pub enum VarRefPrefix {
    This = "#",
    Payload = "$",
    System = "@"
}

impl VarRefPrefix {
    pub fn process(&self, location: &DataLocation, context: &mut ProgramContext) -> Option<Wasm> {
        match self {
            VarRefPrefix::This => match context.get_this_type() {
                Some(this_var) => Some(Wasm::typed(this_var.expr_type.clone(), Wat::get_local(this_var.wasm_name.as_str()))),
                None => {
                    context.error(location, "no `this` value can be referenced in this context");
                    None
                }
            },
            VarRefPrefix::Payload => match context.get_payload_type() {
                Some(payload_var) => Some(Wasm::typed(payload_var.expr_type.clone(), Wat::get_local(payload_var.wasm_name.as_str()))),
                None => {
                    context.error(location, "no `payload` value can be referenced in this context");
                    None
                }
            },
            VarRefPrefix::System => {
                Some(Wasm::typed(Type::System, vec![]))
            },
        }
    }
}