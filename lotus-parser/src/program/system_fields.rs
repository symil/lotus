use crate::{generation::{LOG_BOOL_FUNC_NAME, LOG_FLOAT_FUNC_NAME, LOG_INT_FUNC_NAME, MEMORY_ALLOC_FUNC_NAME, MEMORY_FREE_FUNC_NAME, MEM_ALLOC_FUNC_NAME, MEM_FREE_FUNC_NAME, ToWat, ToWatVec, VALUE_BYTE_SIZE, Wat}, items::{ArgumentList, Identifier}, wat};
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

pub fn process_system_method_call(method_name: &Identifier, arguments: &ArgumentList, context: &mut ProgramContext) -> Option<Wasm> {
    let (arguments, return_type, wat) = match method_name.as_str() {
        "log" => (vec![Type::Any(0)], Type::Void, wat![""]),
        "wasm_memory_grow" => (vec![Type::Integer], Type::Integer, wat!["memory.grow"]),
        "alloc" => (vec![Type::Integer], Type::int_pointer(), Wat::call_from_stack(MEMORY_ALLOC_FUNC_NAME)),
        "free" => (vec![Type::int_pointer()], Type::Void, Wat::call_from_stack(MEMORY_FREE_FUNC_NAME)),
        _ => return None
    };

    Some(Wasm::typed(Type::Function(arguments, Box::new(return_type)), match wat.is_empty() {
        true => vec![],
        false => vec![wat]
    }))
}

pub fn post_process_system_method_call(method_name: &Identifier, arg_types: &[Type], context: &mut ProgramContext) -> Vec<Wat> {
    let wat = match method_name.as_str() {
        "log" => match arg_types[0] {
            Type::Void => unreachable!(),
            Type::System => unreachable!(),
            Type::Boolean => Wat::call_from_stack(LOG_BOOL_FUNC_NAME),
            Type::Integer => Wat::call_from_stack(LOG_INT_FUNC_NAME),
            Type::Float => Wat::call_from_stack(LOG_FLOAT_FUNC_NAME),
            Type::String => todo!(),
            Type::Null => Wat::call_from_stack(LOG_INT_FUNC_NAME),
            Type::TypeId => Wat::call_from_stack(LOG_INT_FUNC_NAME),
            Type::Struct(_) => todo!(),
            Type::Pointer(_) => Wat::call_from_stack(LOG_INT_FUNC_NAME),
            Type::Array(_) => todo!(),
            Type::Function(_, _) => todo!(),
            Type::Any(_) => unreachable!(),
        },
        _ => unreachable!()
    };

    vec![wat]
}