use crate::{generation::{wat, Wat, ToWat, ToWatVec}, items::Identifier};
use super::{ProgramContext, Type, Wasm};

const INT_LOG4 : &'static str = "__int_log4";
const INT_NEXT_POWER_OF_4 : &'static str = "__int_next_power_of_4";
const INT_POW : &'static str = "__int_pow";

pub fn process_integer_field_access(field_name: &Identifier, context: &mut ProgramContext) -> Option<Wasm> {
    match field_name.as_str() {
        _ => None
    }
}

pub fn process_integer_method_call(method_name: &Identifier, context: &mut ProgramContext) -> Option<Wasm> {
    let (arguments, return_type, wat) = match method_name.as_str() {
        "clz" => (vec![], Type::Integer, wat!["i32.clz"]),
        "ctz" => (vec![], Type::Integer, wat!["i32.ctz"]),
        "log4" => (vec![], Type::Integer, Wat::call_from_stack(INT_LOG4)),
        "next_power_of_4" => (vec![], Type::Integer, Wat::call_from_stack(INT_NEXT_POWER_OF_4)),
        "pow" => (vec![Type::Integer], Type::Integer, Wat::call_from_stack(INT_POW)),
        _ => return None
    };

    Some(Wasm::typed(Type::Function(arguments, Box::new(return_type)), wat))
}