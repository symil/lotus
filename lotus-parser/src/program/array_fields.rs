use crate::{generation::{ARRAY_GET_I32_FUNC_NAME, ARRAY_LENGTH_FUNC_NAME, Wat}, items::{Expression, Identifier}};
use super::{ProgramContext, Type, Wasm};

pub fn process_array_field_access(item_type: &Type, field_name: &Identifier, context: &mut ProgramContext) -> Option<Wasm> {
    match field_name.as_str() {
        _ => None
    }
}

pub fn process_array_method_call(item_type: &Type, method_name: &Identifier, context: &mut ProgramContext) -> Option<Wasm> {
    let (wasm_name, arguments, return_type) = match method_name.as_str() {
        "len" => (ARRAY_LENGTH_FUNC_NAME, vec![], Type::Integer),
        "get" => (ARRAY_GET_I32_FUNC_NAME, vec![Type::Integer], item_type.clone()),
        _ => return None
    };

    Some(Wasm::typed(Type::function(arguments, return_type), Wat::call_no_arg(wasm_name)))
}