use crate::{generation::{wat, Wat, ToWat, ToWatVec}, items::Identifier};
use super::{ProgramContext, Type, Wasm};

const INT_LOG4_FUNC_NAME : &'static str = "std_int_log4";

pub fn process_integer_field_access(field_name: &Identifier, context: &mut ProgramContext) -> Option<Wasm> {
    match field_name.as_str() {
        _ => None
    }
}

pub fn process_integer_method_call(method_name: &Identifier, context: &mut ProgramContext) -> Option<Wasm> {
    let (arguments, return_type, wat) = match method_name.as_str() {
        "clz" => (vec![], Type::Integer, wat!["i32.clz"]),
        "ctz" => (vec![], Type::Integer, wat!["i32.ctz"]),
        "log4" => (vec![], Type::Integer, Wat::call_no_arg(INT_LOG4_FUNC_NAME)),
        _ => return None
    };

    Some(Wasm::typed(Type::Function(arguments, Box::new(return_type)), wat))
}