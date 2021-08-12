use crate::{generation::{LOG_I32_FUNC_NAME, MEM_ALLOC_FUNC_NAME, MEM_FREE_FUNC_NAME, ToWat, ToWatVec, VALUE_BYTE_SIZE, Wat}, items::Identifier, wat};
use super::{ProgramContext, Type, Wasm};

pub fn process_system_field_access(field_name: &Identifier, context: &mut ProgramContext) -> Option<Wasm> {
    match field_name.as_str() {
        "memory" => Some(Wasm::typed(
            Type::pointer(Type::Integer),
            Wat::const_i32(0)
        )),
        _ => None
    }
}

pub fn process_system_method_call(method_name: &Identifier, context: &mut ProgramContext) -> Option<Wasm> {
    let (arguments, return_type, wasm_name) = match method_name.as_str() {
        "log_int" => (vec![Type::Integer], Type::Void, LOG_I32_FUNC_NAME),
        _ => return None
    };

    Some(Wasm::typed(Type::Function(arguments, Box::new(return_type)), vec![Wat::call_no_arg(wasm_name)]))
}