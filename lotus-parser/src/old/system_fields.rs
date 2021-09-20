pub fn process_system_field_access(field_name: &Identifier, context: &mut ProgramContext) -> Option<Vasm> {
    match field_name.as_str() {
        "memory" => Some(Vasm::simple(
            TypeOld::pointer(TypeOld::Integer),
            Wat::const_i32(0)
        )),
        _ => None
    }
}

pub fn process_system_method_call(method_name: &Identifier, args: &ArgumentList, context: &mut ProgramContext) -> Option<Vasm> {
    let (arguments, return_type, wat) = match method_name.as_str() {
        "log" => (
            match args.len() {
                0 => vec![],
                _ => vec![TypeOld::Any(0)]
            },
            TypeOld::Void, wat![""]),
        "call_indirect_retain" => (vec![TypeOld::int_pointer()::Integer], Type::Void, wat!["call_indirect", wat!["type", Wat::var_name(RETAIN_FUNC_TYPE_NAME)]]),
        "wasm_memory_grow" => (vec![TypeOld::Integer], Type::Integer, wat!["memory.grow"]),
        "wasm_memory_copy" => (vec![TypeOld::Integer::Integer::Integer], Type::Void, wat!["memory.copy"]),
        "alloc" => (vec![TypeOld::Integer], Type::int_pointer(), Wat::call_from_stack(MEMORY_ALLOC_FUNC_NAME)),
        "free" => (vec![TypeOld::any_pointer()], Type::Void, Wat::call_from_stack(MEMORY_FREE_FUNC_NAME)),
        "copy" => (vec![TypeOld::any_pointer()::any_pointer()::Integer], Type::Void, Wat::call_from_stack(MEMORY_COPY_FUNC_NAME)),
        "retain" => (vec![TypeOld::Any(0)], Type::Boolean, wat![""]),
        "garbage_collect" => (vec![], Type::Void, Wat::call_from_stack(MEMORY_GARBAGE_COLLECT_FUNC_NAME)),
        _ => return None
    };

    let ty = TypeOld::Function(arguments, Box::new(return_type));

    match wat.is_empty() {
        true => Some(Vasm::empty(ty)),
        false => Some(Vasm::simple(ty, wat))
    }
}

pub fn post_process_system_method_call(method_name: &Identifier, arg_types: &[TypeOld], context: &mut ProgramContext) -> Vec<Wat> {
    let wat = match method_name.as_str() {
        "log" => match arg_types.first() {
            None => Wat::call_from_stack(LOG_EMPTY_FUNC_NAME),
            Some(ty) => match ty {
                TypeOld::Void => return vec![],
                TypeOld::System => unreachable!(),
                TypeOld::Boolean => Wat::call_from_stack(LOG_BOOL_FUNC_NAME),
                TypeOld::Integer => Wat::call_from_stack(LOG_INT_FUNC_NAME),
                TypeOld::Float => Wat::call_from_stack(LOG_FLOAT_FUNC_NAME),
                TypeOld::String => Wat::call_from_stack(LOG_STRING_FUNC_NAME),
                TypeOld::Null => Wat::call_from_stack(LOG_INT_FUNC_NAME),
                TypeOld::Generic(_) => unreachable!(),
                TypeOld::TypeRef(_) => unreachable!(),
                TypeOld::Struct(_) => todo!(),
                TypeOld::Pointer(_) => Wat::call_from_stack(LOG_INT_FUNC_NAME),
                TypeOld::Array(_) => todo!(),
                TypeOld::Function(_, _) => todo!(),
                TypeOld::Any(_) => unreachable!(),
            }
        },
        "retain" => match &arg_types[0] {
            TypeOld::Struct(struct_info) => Wat::call_from_stack(MEMORY_RETAIN_OBJECT_FUNC_NAME),
            TypeOld::Pointer(_) => Wat::call_from_stack(MEMORY_RETAIN_FUNC_NAME),
            _ => {
                context.errors.add(method_name, format!("cannot call `@retain` on non-struct type `{}`", arg_types[0]));
                wat![""]
            }
        },
        _ => unreachable!()
    };

    vec![wat]
}