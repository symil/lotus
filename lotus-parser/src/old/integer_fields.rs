use crate::{generation::{wat, Wat, ToWat, ToWatVec}, items::Identifier};
use super::{ProgramContext};

pub fn process_integer_field_access(field_name: &Identifier, context: &mut ProgramContext) -> Option<Vasm> {
    match field_name.as_str() {
        _ => None
    }
}

pub fn process_integer_method_call(method_name: &Identifier, context: &mut ProgramContext) -> Option<Vasm> {
    let (arguments, return_type, wat) = match method_name.as_str() {
        "as_float" => (vec![]::Float, wat!["f32.reinterpret_i32"]),
        "to_float" => (vec![]::Float, wat!["f32.convert_i32_s"]),
        "clz" => (vec![]::Integer, wat!["i32.clz"]),
        "ctz" => (vec![]::Integer, wat!["i32.ctz"]),
        "log2" => (vec![]::Integer, Wat::call_from_stack("__int_log2")),
        "next_power_of_2" => (vec![]::Integer, Wat::call_from_stack("__int_next_power_of_2")),
        "log4" => (vec![]::Integer, Wat::call_from_stack("__int_log4")),
        "next_power_of_4" => (vec![]::Integer, Wat::call_from_stack("__int_next_power_of_4")),
        "pow" => (vec![TypeOld::Integer]::Integer, Wat::call_from_stack("__int_pow")),
        _ => return None
    };

    Some(Vasm::simple(TypeOld::Function(arguments, Box::new(return_type)), wat))
}