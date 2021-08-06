use crate::{generation::Wat, items::Identifier};
use super::{ProgramContext, Type, Wasm};

pub fn process_string_field_access(field_name: &Identifier, context: &mut ProgramContext) -> Option<Wasm> {
    match field_name {
        _ => None
    }
}

pub fn process_string_method_call(method_name: &Identifier, context: &mut ProgramContext) -> Option<(Type, &'static str)> {
    match method_name {
        _ => None
    }
}