use crate::{generation::{Wat}, items::{Expression, Identifier}};
use super::{ProgramContext, Type, Wasm};

pub const ARRAY_ALLOC_FUNC_NAME : &'static str = "__array_alloc";
pub const ARRAY_GET_ITEM_FUNC_NAME : &'static str = "__array_get_item";
pub const ARRAY_SET_ITEM_FUNC_NAME : &'static str = "__array_set_item";
pub const ARRAY_GET_LENGTH_FUNC_NAME : &'static str = "__array_get_length";
pub const ARRAY_GET_BODY_FUNC_NAME : &'static str = "__array_get_body";
pub const ARRAY_SET_BODY_ITEM_FUNC_NAME : &'static str = "__array_set_body_item";

pub fn process_array_field_access(item_type: &Type, field_name: &Identifier, context: &mut ProgramContext) -> Option<Wasm> {
    match field_name.as_str() {
        _ => None
    }
}

pub fn process_array_method_call(item_type: &Type, method_name: &Identifier, context: &mut ProgramContext) -> Option<Wasm> {
    let (arguments, return_type, func_name) = match method_name.as_str() {
        "len" => (vec![], Type::Integer, ARRAY_GET_LENGTH_FUNC_NAME),
        "get" => (vec![Type::Integer], item_type.clone(), ARRAY_GET_ITEM_FUNC_NAME),
        _ => return None
    };

    Some(Wasm::simple(Type::function(arguments, return_type), Wat::call_from_stack(func_name)))
}