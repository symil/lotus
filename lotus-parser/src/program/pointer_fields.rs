use crate::{generation::Wat, items::{Expression, Identifier}};
use super::{ProgramContext, Type, Wasm};

pub fn process_pointer_field_access(pointed_type: &Type, field_name: &Identifier, context: &mut ProgramContext) -> Option<Wasm> {
    match field_name {
        _ => None
    }
}

pub fn process_pointer_method_call(pointed_type: &Type, method_name: &Identifier, context: &mut ProgramContext) -> Option<Wasm> {
    match method_name {
        _ => None
    }
}