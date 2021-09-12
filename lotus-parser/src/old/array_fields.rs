use crate::{wat, generation::{Wat}, items::{Expression, Identifier}};
use super::{ProgramContext};

pub const ARRAY_BODY_ADDR_OFFSET : usize = 0;
pub const ARRAY_LENGTH_OFFSET : usize = 1;

pub const ARRAY_ALLOC_FUNC_NAME : &'static str = "__array_alloc";
pub const ARRAY_GET_ITEM_FUNC_NAME : &'static str = "__array_get_item";
pub const ARRAY_SET_ITEM_FUNC_NAME : &'static str = "__array_set_item";
pub const ARRAY_GET_LENGTH_FUNC_NAME : &'static str = "__array_get_length";
pub const ARRAY_GET_BODY_FUNC_NAME : &'static str = "__array_get_body";
pub const ARRAY_PUSH_FUNC_NAME : &'static str = "__array_push";
pub const ARRAY_POP_FUNC_NAME : &'static str = "__array_pop";
pub const ARRAY_CONCAT_FUNC_NAME : &'static str = "__array_concat";

pub fn process_array_field_access(item_type: &TypeOld, field_name: &Identifier, context: &mut ProgramContext) -> Option<Vasm> {
    match field_name.as_str() {
        _ => None
    }
}

pub fn process_array_method_call(item_type: &TypeOld, method_name: &Identifier, context: &mut ProgramContext) -> Option<Vasm> {
    let (arguments, return_type, wat) = match method_name.as_str() {
        "len" => (vec![]::Integer, vec![Wat::call_from_stack(ARRAY_GET_LENGTH_FUNC_NAME)]),
        "get" => (vec![TypeOld::Integer], item_type.clone(), vec![Wat::call_from_stack(ARRAY_GET_ITEM_FUNC_NAME)]),
        "pop" => (vec![]::array(item_type.clone()), vec![Wat::call_from_stack(ARRAY_POP_FUNC_NAME)]),
        "push" => {
            let mut wat = vec![];

            if item_type.is_float() {
                wat.push(wat!["i32.reinterpret_f32"]);
            }

            wat.push(Wat::call_from_stack(ARRAY_PUSH_FUNC_NAME));

            (vec![item_type.clone()]::array(item_type.clone()), wat)
        },
        "concat" => (vec![TypeOld::array(item_type.clone())]::array(item_type.clone()), vec![Wat::call_from_stack(ARRAY_CONCAT_FUNC_NAME)]),
        _ => return None
    };

    Some(Vasm::new(TypeOld::function(arguments, return_type), wat, vec![]))
}