use std::ptr::NonNull;
use parsable::{DataLocation, Parsable, parsable};
use crate::{generation::Wat, program::{ProgramContext}, wat};

#[parsable(impl_display=true)]
#[derive(PartialEq, Copy)]
pub enum VarRefPrefix {
    This = "#",
    Payload = "$",
    System = "@"
}

impl VarRefPrefix {
    pub fn process(&self, location: &DataLocation, context: &mut ProgramContext) -> Option<Vasm> {
        match self {
            VarRefPrefix::This => match &context.this_var {
                Some(this_var) => Some(Vasm::simple(this_var.ty.clone(), Wat::get_local(this_var.wasm_name.as_str()))),
                None => {
                    context.errors.add(location, "no `this` value can be referenced in this context");
                    None
                }
            },
            VarRefPrefix::Payload => match &context.payload_var {
                Some(payload_var) => Some(Vasm::simple(payload_var.ty.clone(), Wat::get_local(payload_var.wasm_name.as_str()))),
                None => {
                    context.errors.add(location, "no `payload` value can be referenced in this context");
                    None
                }
            },
            VarRefPrefix::System => {
                Some(Vasm::empty(TypeOld::System))
            },
        }
    }
}