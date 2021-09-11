use crate::{generation::Wat, items::{Expression, Identifier}};
use super::{ProgramContext};

pub fn process_float_field_access(field_name: &Identifier, context: &mut ProgramContext) -> Option<Vasm> {
    match field_name {
        _ => None
    }
}

pub fn process_float_method_call(method_name: &Identifier, context: &mut ProgramContext) -> Option<Vasm> {
    match method_name {
        _ => None
    }
}