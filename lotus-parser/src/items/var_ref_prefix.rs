use std::ptr::NonNull;
use parsable::{DataLocation, Parsable, parsable};
use crate::{program::{BuiltinType, ProgramContext, VI, Vasm}, wat};

#[parsable(impl_display=true)]
#[derive(PartialEq, Copy)]
pub enum VarRefPrefix {
    This = "#",
    Payload = "$",
    System = "@"
}

impl VarRefPrefix {
    pub fn process(&self, location: &DataLocation, context: &mut ProgramContext) -> Option<Vasm> {
        let mut result = None;

        match self {
            VarRefPrefix::This => match &context.current_function {
                Some(function_blueprint) => match &function_blueprint.borrow().this_arg {
                    Some(this_var) => {
                        result = Some(Vasm::new(this_var.ty.clone(), vec![], vec![VI::get(this_var)]));
                    },
                    None => context.errors.add(location, "no `this` value can be referenced in this context")
                }
                None => context.errors.add(location, "no `this` value can be referenced in this context")
            },
            VarRefPrefix::Payload => match &context.current_function {
                Some(function_blueprint) => match &function_blueprint.borrow().payload_arg {
                    Some(payload_var) => {
                        result = Some(Vasm::new(payload_var.ty.clone(), vec![], vec![VI::get(payload_var)]));
                    },
                    None => context.errors.add(location, "no `payload` value can be referenced in this context")
                }
                None => context.errors.add(location, "no `payload` value can be referenced in this context")
            },
            VarRefPrefix::System => {
                result = Some(Vasm::new(context.get_builtin_type(BuiltinType::System, vec![]), vec![], vec![]))
            },
        };

        result
    }
}