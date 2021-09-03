use crate::{generation::{LOG_BOOL_FUNC_NAME, LOG_EMPTY_FUNC_NAME, LOG_FLOAT_FUNC_NAME, LOG_INT_FUNC_NAME, LOG_STRING_FUNC_NAME, MEMORY_ALLOC_FUNC_NAME, MEMORY_COPY_FUNC_NAME, MEMORY_FREE_FUNC_NAME, MEMORY_GARBAGE_COLLECT_FUNC_NAME, MEMORY_RETAIN_FUNC_NAME, MEMORY_RETAIN_OBJECT_FUNC_NAME, RETAIN_FUNC_TYPE_NAME, VALUE_BYTE_SIZE, Wat}, items::{ArgumentList, Identifier}, wat};
use super::{ProgramContext, Type, Wasm};

pub fn process_system_field_access(field_name: &Identifier, context: &mut ProgramContext) -> Option<Wasm> {
    match field_name.as_str() {
        "memory" => Some(Wasm::simple(
            Type::pointer(Type::Integer),
            Wat::const_i32(0)
        )),
        _ => None
    }
}

pub fn process_system_method_call(method_name: &Identifier, args: &ArgumentList, context: &mut ProgramContext) -> Option<Wasm> {
    let (arguments, return_type, wat) = match method_name.as_str() {
        "log" => (
            match args.len() {
                0 => vec![],
                _ => vec![Type::Any(0)]
            },
            Type::Void, wat![""]),
        "call_indirect_retain" => (vec![Type::int_pointer(), Type::Integer], Type::Void, wat!["call_indirect", wat!["type", Wat::var_name(RETAIN_FUNC_TYPE_NAME)]]),
        "wasm_memory_grow" => (vec![Type::Integer], Type::Integer, wat!["memory.grow"]),
        "wasm_memory_copy" => (vec![Type::Integer, Type::Integer, Type::Integer], Type::Void, wat!["memory.copy"]),
        "alloc" => (vec![Type::Integer], Type::int_pointer(), Wat::call_from_stack(MEMORY_ALLOC_FUNC_NAME)),
        "free" => (vec![Type::any_pointer()], Type::Void, Wat::call_from_stack(MEMORY_FREE_FUNC_NAME)),
        "copy" => (vec![Type::any_pointer(), Type::any_pointer(), Type::Integer], Type::Void, Wat::call_from_stack(MEMORY_COPY_FUNC_NAME)),
        "retain" => (vec![Type::Any(0)], Type::Boolean, wat![""]),
        "garbage_collect" => (vec![], Type::Void, Wat::call_from_stack(MEMORY_GARBAGE_COLLECT_FUNC_NAME)),
        _ => return None
    };

    let ty = Type::Function(arguments, Box::new(return_type));

    match wat.is_empty() {
        true => Some(Wasm::empty(ty)),
        false => Some(Wasm::simple(ty, wat))
    }
}

pub fn post_process_system_method_call(method_name: &Identifier, arg_types: &[Type], context: &mut ProgramContext) -> Vec<Wat> {
    let wat = match method_name.as_str() {
        "log" => match arg_types.first() {
            None => Wat::call_from_stack(LOG_EMPTY_FUNC_NAME),
            Some(ty) => match ty {
                Type::Void => return vec![],
                Type::System => unreachable!(),
                Type::Boolean => Wat::call_from_stack(LOG_BOOL_FUNC_NAME),
                Type::Integer => Wat::call_from_stack(LOG_INT_FUNC_NAME),
                Type::Float => Wat::call_from_stack(LOG_FLOAT_FUNC_NAME),
                Type::String => Wat::call_from_stack(LOG_STRING_FUNC_NAME),
                Type::Null => Wat::call_from_stack(LOG_INT_FUNC_NAME),
                Type::Generic(_) => unreachable!(),
                Type::TypeRef(_) => unreachable!(),
                Type::Struct(_) => todo!(),
                Type::Pointer(_) => Wat::call_from_stack(LOG_INT_FUNC_NAME),
                Type::Array(_) => todo!(),
                Type::Function(_, _) => todo!(),
                Type::Any(_) => unreachable!(),
            }
        },
        "retain" => match &arg_types[0] {
            Type::Struct(struct_info) => Wat::call_from_stack(MEMORY_RETAIN_OBJECT_FUNC_NAME),
            Type::Pointer(_) => Wat::call_from_stack(MEMORY_RETAIN_FUNC_NAME),
            _ => {
                context.errors.add(method_name, format!("cannot call `@retain` on non-struct type `{}`", arg_types[0]));
                wat![""]
            }
        },
        _ => unreachable!()
    };

    vec![wat]
}