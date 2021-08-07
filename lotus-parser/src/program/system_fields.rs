use crate::{generation::{LOG_I32_FUNC_NAME, MEM_ALLOC_FUNC_NAME, MEM_FREE_FUNC_NAME, Wat}, items::Identifier};
use super::{ProgramContext, Type, Wasm};

pub fn process_system_field_access(field_name: &Identifier, context: &mut ProgramContext) -> Option<Wasm> {
    match field_name.as_str() {
        "memory" => Some(Wasm::typed(
            Type::Pointer,
            Wat::const_i32(0)
        )),
        _ => None
    }
}

pub fn process_system_method_call(method_name: &Identifier, context: &mut ProgramContext) -> Option<(Type, &'static str)> {
    match method_name.as_str() {
        "alloc" => Some((
            Type::function(vec![Type::Integer], Type::Pointer),
            MEM_ALLOC_FUNC_NAME
        )),
        "free" => Some((
            Type::function(vec![Type::Pointer], Type::Void),
            MEM_FREE_FUNC_NAME
        )),
        "log_ptr" => Some((
            Type::function(vec![Type::Pointer], Type::Void),
            LOG_I32_FUNC_NAME
        )),
        _ => None
    }
}