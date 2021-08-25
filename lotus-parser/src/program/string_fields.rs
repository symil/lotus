use crate::{generation::Wat, items::Identifier};
use super::{ProgramContext, Type, Wasm};

pub const STRING_ALLOC_FUNC_NAME : &'static str = "__string_alloc";
pub const STRING_SET_CHAR_FUNC_NAME : &'static str = "__string_set_char";
pub const STRING_GET_CHAR_FUNC_NAME : &'static str = "__string_get_char";
pub const STRING_GET_LENGTH_FUNC_NAME : &'static str = "__string_get_length";
pub const STRING_EQUAL_FUNC_NAME : &'static str = "__string_equal";
pub const STRING_CONCAT_FUNC_NAME : &'static str = "__string_concat";

pub fn process_string_field_access(field_name: &Identifier, context: &mut ProgramContext) -> Option<Wasm> {
    match field_name {
        _ => None
    }
}

pub fn process_string_method_call(method_name: &Identifier, context: &mut ProgramContext) -> Option<Wasm> {
    let (arguments, return_type, func_name) = match method_name.as_str() {
        "len" => (vec![], Type::Integer, STRING_GET_LENGTH_FUNC_NAME),
        _ => return None
    };

    Some(Wasm::typed(Type::Function(arguments, Box::new(return_type)), vec![Wat::call_from_stack(func_name)]))
}