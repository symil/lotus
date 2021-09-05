use crate::{generation::{wat, Wat, ToWat, ToWatVec}, items::Identifier};
use super::{ProgramContext, TypeOld, Wasm};

pub fn process_integer_field_access(field_name: &Identifier, context: &mut ProgramContext) -> Option<Wasm> {
    match field_name.as_str() {
        _ => None
    }
}

pub fn process_integer_method_call(method_name: &Identifier, context: &mut ProgramContext) -> Option<Wasm> {
    let (arguments, return_type, wat) = match method_name.as_str() {
        "as_float" => (vec![], TypeOld::Float, wat!["f32.reinterpret_i32"]),
        "to_float" => (vec![], TypeOld::Float, wat!["f32.convert_i32_s"]),
        "clz" => (vec![], TypeOld::Integer, wat!["i32.clz"]),
        "ctz" => (vec![], TypeOld::Integer, wat!["i32.ctz"]),
        "log2" => (vec![], TypeOld::Integer, Wat::call_from_stack("__int_log2")),
        "next_power_of_2" => (vec![], TypeOld::Integer, Wat::call_from_stack("__int_next_power_of_2")),
        "log4" => (vec![], TypeOld::Integer, Wat::call_from_stack("__int_log4")),
        "next_power_of_4" => (vec![], TypeOld::Integer, Wat::call_from_stack("__int_next_power_of_4")),
        "pow" => (vec![TypeOld::Integer], TypeOld::Integer, Wat::call_from_stack("__int_pow")),
        _ => return None
    };

    Some(Wasm::simple(TypeOld::Function(arguments, Box::new(return_type)), wat))
}