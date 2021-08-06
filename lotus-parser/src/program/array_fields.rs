use crate::{generation::{ARRAY_GET_FUNC_NAME, ARRAY_LENGTH_FUNC_NAME, Wat}, items::{Expression, Identifier}};
use super::{ProgramContext, Type, Wasm};

pub fn process_array_field_access(item_type: &Type, field_name: &Identifier, context: &mut ProgramContext) -> Option<Wasm> {
    match field_name.as_str() {
        _ => None
    }
}

pub fn process_array_method_call(item_type: &Type, method_name: &Identifier, context: &mut ProgramContext) -> Option<(Type, &'static str)> {
    match method_name.as_str() {
        "len" => Some((
            Type::function(vec![], Type::int()),
            ARRAY_LENGTH_FUNC_NAME,
        )),
        "get" => Some((
            Type::function(vec![Type::int()], item_type.clone()),
            ARRAY_GET_FUNC_NAME,
        )),
        _ => None
    }
}