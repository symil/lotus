use parsable::{Parsable, parsable};

use crate::program::{ProgramContext, Type, Wasm};

#[parsable(impl_display=true)]
#[derive(PartialEq, Copy)]
pub enum VarRefPrefix {
    This = "#",
    Payload = "$",
    System = "@"
}

impl VarRefPrefix {
    pub fn process<T : Parsable>(&self, location: &T, context: &mut ProgramContext) -> Option<Type> {
        match self {
            VarRefPrefix::This => {
                if context.get_this_type().is_none() {
                    context.error(location, "no `this` value can be referenced in this context");
                }

                context.get_this_type()
            },
            VarRefPrefix::Payload => {
                if context.get_payload_type().is_none() {
                    context.error(location, "no `payload` value can be referenced in this context");
                }

                context.get_payload_type()
            },
            VarRefPrefix::System => {
                Some(Type::system())
            },
        }
    }
}