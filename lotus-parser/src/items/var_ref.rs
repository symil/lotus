use std::collections::HashMap;

use parsable::parsable;
use crate::{generation::{Wat}, program::{AccessType, OBJECT_HEADER_SIZE, ProgramContext, Type, VariableKind, Wasm, post_process_system_method_call, process_array_field_access, process_array_method_call, process_boolean_field_access, process_boolean_method_call, process_float_field_access, process_float_method_call, process_integer_field_access, process_integer_method_call, process_pointer_field_access, process_pointer_method_call, process_string_field_access, process_string_method_call, process_system_field_access, process_system_method_call}};
use super::{ArgumentList, Identifier, VarRefPrefix};

#[parsable]
pub struct VarRef {
    pub name: Identifier,
    pub arguments: Option<ArgumentList>
}

impl VarRef {
    pub fn process_as_field(&self, parent_type: &Type, access_type: AccessType, context: &mut ProgramContext) -> Option<Wasm> {
        match &self.arguments {
            Some(arguments) => process_method_call(parent_type, &self.name, arguments, access_type, context),
            None => process_field_access(parent_type, &self.name, access_type, context)
        }
    }

    pub fn process_as_variable(&self, access_type: AccessType, context: &mut ProgramContext) -> Option<Wasm> {
        match &self.arguments {
            Some(arguments) => match context.get_function_by_name(&self.name) {
                Some(function_annotation) => {
                    process_function_call(None, &function_annotation.get_type(), vec![Wat::call_from_stack(&function_annotation.wasm_name)], arguments, access_type, context)
                },
                None => {
                    context.error(&self.name, format!("undefined function `{}`", &self.name));
                    None
                },
            },
            None => match context.get_var_info(&self.name) {
                Some(var_info) => match access_type {
                    AccessType::Get => Some(Wasm::simple(var_info.ty.clone(), var_info.get_to_stack())),
                    AccessType::Set(_) => Some(Wasm::simple(var_info.ty.clone(), var_info.set_from_stack())),
                },
                None => {
                    context.error(&self.name, format!("undefined variable `{}`", &self.name));
                    None
                },
            }
        }
    }
}

pub fn process_field_access(parent_type: &Type, field_name: &Identifier, access_type: AccessType, context: &mut ProgramContext) -> Option<Wasm> {
    if let AccessType::Set(set_location) = access_type {
        match parent_type {
            Type::Struct(_) => {},
            _ => {
                context.error(set_location, format!("cannot set field of non-struct value"));
                return None;
            }
        }
    }

    let result = match parent_type {
        Type::Void => None,
        Type::Null => None,
        Type::System => process_system_field_access(field_name, context),
        Type::Boolean => process_boolean_field_access(field_name, context),
        Type::Integer => process_integer_field_access(field_name, context),
        Type::Float => process_float_field_access(field_name, context),
        Type::String => process_string_field_access(field_name, context),
        Type::TypeId => None,
        Type::Pointer(pointed_type) => process_pointer_field_access(pointed_type, field_name, context),
        Type::Array(item_type) => process_array_field_access(item_type, field_name, context),
        Type::Struct(struct_info) => {
            if field_name.is("_") {
                if let AccessType::Set(set_location) = access_type {
                    context.error(set_location, format!("cannot set special field `_`"));
                    return None;
                }


                // special case: `_` refers to the value itself rather than a field
                // e.g `#foo` means `self.foo`, but `#_` means `self`
                Some(Wasm::empty(parent_type.clone()))
            } else if let Some(struct_annotation) = context.get_struct_by_id(struct_info.id) {
                if let Some(field) = struct_annotation.fields.get(field_name) {
                    let func_name = match access_type {
                        AccessType::Get => field.ty.pointer_get_function_name(),
                        AccessType::Set(_) => field.ty.pointer_set_function_name(),
                    };

                    Some(Wasm::simple(
                        field.ty.clone(),
                        Wat::call(func_name, vec![Wat::const_i32(field.offset + OBJECT_HEADER_SIZE)])
                    ))
                } else {
                    None
                }
            } else {
                None
            }
        },
        Type::Function(_, _) => None,
        Type::Any(_) => None,
        
    };

    if result.is_none() {
        context.error(field_name, format!("type `{}` has no field `{}`", parent_type, field_name));
    }

    result
}

pub fn process_method_call(parent_type: &Type, method_name: &Identifier, arguments: &ArgumentList, access_type: AccessType, context: &mut ProgramContext) -> Option<Wasm> {
    let mut result = None;

    let method_info : Option<Wasm> = match parent_type {
        Type::Void => None,
        Type::Null => None,
        Type::TypeId => None,
        Type::System => process_system_method_call(method_name, arguments, context),
        Type::Boolean => process_boolean_method_call(method_name, context),
        Type::Integer => process_integer_method_call(method_name, context),
        Type::Float => process_float_method_call(method_name, context),
        Type::String => process_string_method_call(method_name, context),
        Type::Pointer(pointed_type) => process_pointer_method_call(pointed_type, method_name, context),
        Type::Array(item_type) => process_array_method_call(item_type, method_name, context),
        Type::Struct(struct_info) => {
            if let Some(struct_annotation) = context.get_struct_by_id(struct_info.id) {
                if let Some(method) = struct_annotation.user_methods.get(method_name) {
                    Some(Wasm::simple(method.get_type(), Wat::call_from_stack(&method.wasm_name)))
                } else {
                    None
                }
            } else {
                None
            }
        },
        Type::Function(_, _) => None,
        Type::Any(_) => None,
    };

    if let Some(method_wasm) = method_info {
        result = process_function_call(Some(method_name), &method_wasm.ty, method_wasm.wat, arguments, access_type, context);
    } else if !parent_type.is_void() {
        context.error(method_name, format!("type `{}` has no method `{}`", parent_type, method_name));
    }

    result
}

pub fn process_function_call(system_method_name: Option<&Identifier>, function_type: &Type, mut function_call: Vec<Wat>, arguments: &ArgumentList, access_type: AccessType, context: &mut ProgramContext) -> Option<Wasm> {
    if let AccessType::Set(set_location) = access_type  {
        context.error(set_location, format!("cannot set result of a function call"));
        return None;
    }

    let (expected_arguments, return_type) = function_type.as_function();

    let mut ok = true;
    let mut source = vec![];
    let mut argument_types = vec![];
    let mut anonymous_types = HashMap::new();

    if arguments.len() != expected_arguments.len() {
        let s = if expected_arguments.len() > 1 { "s" } else { "" };
        context.error(arguments, format!("expected {} argument{}, got {}", expected_arguments.len(), s, arguments.as_vec().len()));
        ok = false;
    }

    for (i, (arg_expr, expected_type)) in arguments.as_vec().iter().zip(expected_arguments.iter()).enumerate() {
        if let Some(arg_wasm) = arg_expr.process(context) {
            argument_types.push(arg_wasm.ty.clone());

            if expected_type.is_assignable(&arg_wasm.ty, context, &mut anonymous_types) {
                source.push(arg_wasm);
            } else {
                context.error(arg_expr, format!("argument #{}: expected `{}`, got `{}`", i + 1, expected_type, &arg_wasm.ty));
                ok = false;
            }
        } else {
            ok = false;
        }
    }

    // Special case: some builtin (system) functions (e.g `@log`) need to know the type of their arguments to generate proper WAT
    // In such case, the WAT generation is delayed until after the arguments have been processed
    // This only happen for a few builtin functions, and can never happen for user-written functions
    if ok && function_call.is_empty() {
        function_call = post_process_system_method_call(system_method_name.unwrap(), &argument_types, context);
    }

    source.push(Wasm::new(Type::Void, function_call, vec![]));

    match ok {
        true => Some(Wasm::merge(return_type.clone(), source)),
        false => None
    }
}