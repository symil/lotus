use std::collections::HashMap;

use parsable::parsable;
use crate::{generation::{Wat}, program::{AccessType, ProgramContext, Type, VariableKind, post_process_system_method_call, process_array_field_access, process_array_method_call, process_boolean_field_access, process_boolean_method_call, process_float_field_access, process_float_method_call, process_integer_field_access, process_integer_method_call, process_pointer_field_access, process_pointer_method_call, process_string_field_access, process_string_method_call, process_system_field_access, process_system_method_call}};
use super::{ArgumentList, Identifier, VarRefPrefix};

#[parsable]
pub struct VarRef {
    pub name: Identifier,
    pub arguments: Option<ArgumentList>
}

impl VarRef {
    pub fn has_side_effects(&self) -> bool {
        self.arguments.is_some()
    }

    pub fn process_as_field(&self, parent_type: &Type, access_type: AccessType, context: &mut ProgramContext) -> Option<Vasm> {
        match &self.arguments {
            Some(arguments) => process_method_call(parent_type, &self.name, arguments, access_type, context),
            None => process_field_access(parent_type, &self.name, access_type, context)
        }
    }

    pub fn process_as_variable(&self, access_type: AccessType, context: &mut ProgramContext) -> Option<Vasm> {
        match &self.arguments {
            Some(arguments) => match context.functions.get_by_name(&self.name) {
                Some(function_blueprint) => {
                    process_function_call(None, &function_blueprint.get_type(), vec![Wat::call_from_stack(&function_blueprint.wasm_name)], arguments, access_type, context)
                },
                None => {
                    context.errors.add(&self.name, format!("undefined function `{}`", &self.name));
                    None
                },
            },
            None => match context.get_var_info(&self.name) {
                Some(var_info) => match access_type {
                    AccessType::Get => Some(Vasm::simple(var_info.ty.clone(), var_info.get_to_stack())),
                    AccessType::Set(_) => Some(Vasm::simple(var_info.ty.clone(), var_info.set_from_stack())),
                },
                None => match context.get_struct_by_name(&self.name) {
                    Some(struct_annotation) => Some(Vasm::empty(TypeOld::TypeRef(struct_annotation.get_struct_info()))),
                    None => {
                        context.errors.add(&self.name, format!("undefined variable or type `{}`", &self.name));
                        None
                    },
                }
            }
        }
    }
}

pub fn process_field_access(parent_type: &TypeOld, field_name: &Identifier, access_type: AccessType, context: &mut ProgramContext) -> Option<Vasm> {
    if let AccessType::Set(set_location) = access_type {
        match parent_type {
            TypeOld::Struct(_) => {},
            _ => {
                context.errors.add(set_location, format!("cannot set field of non-struct value"));
                return None;
            }
        }
    }

    let result = match parent_type {
        TypeOld::Void => None,
        TypeOld::Null => None,
        TypeOld::Generic(_) => None,
        TypeOld::System => process_system_field_access(field_name, context),
        TypeOld::Boolean => process_boolean_field_access(field_name, context),
        TypeOld::Integer => process_integer_field_access(field_name, context),
        TypeOld::Float => process_float_field_access(field_name, context),
        TypeOld::String => process_string_field_access(field_name, context),
        TypeOld::TypeRef(_) => None,
        TypeOld::Pointer(pointed_type) => process_pointer_field_access(pointed_type, field_name, context),
        TypeOld::Array(item_type) => process_array_field_access(item_type, field_name, context),
        TypeOld::Struct(struct_info) => {
            if field_name.is("_") {
                if let AccessType::Set(set_location) = access_type {
                    context.errors.add(set_location, format!("cannot set special field `_`"));
                    return None;
                }


                // special case: `_` refers to the value itself rather than a field
                // e.g `#foo` means `self.foo`, but `#_` means `self`
                Some(Vasm::empty(parent_type.clone()))
            } else if let Some(struct_annotation) = context.get_struct_by_id(struct_info.id) {
                if let Some(field) = struct_annotation.fields.get(field_name) {
                    let func_name = match access_type {
                        AccessType::Get => field.ty.pointer_get_function_name(),
                        AccessType::Set(_) => field.ty.pointer_set_function_name(),
                    };

                    Some(Vasm::simple(
                        field.ty.clone(),
                        Wat::call(func_name, vec![Wat::const_i32(field.offset)])
                    ))
                } else {
                    None
                }
            } else {
                None
            }
        },
        TypeOld::Function(_, _) => None,
        TypeOld::Any(_) => None,
        
    };

    if result.is_none() {
        context.errors.add(field_name, format!("type `{}` has no field `{}`", parent_type, field_name));
    }

    result
}

pub fn process_method_call(parent_type: &TypeOld, method_name: &Identifier, arguments: &ArgumentList, access_type: AccessType, context: &mut ProgramContext) -> Option<Vasm> {
    let mut result = None;

    let method_info : Option<Vasm> = match parent_type {
        TypeOld::Void => None,
        TypeOld::Null => None,
        TypeOld::Generic(_) => None,
        TypeOld::System => process_system_method_call(method_name, arguments, context),
        TypeOld::Boolean => process_boolean_method_call(method_name, context),
        TypeOld::Integer => process_integer_method_call(method_name, context),
        TypeOld::Float => process_float_method_call(method_name, context),
        TypeOld::String => process_string_method_call(method_name, context),
        TypeOld::Pointer(pointed_type) => process_pointer_method_call(pointed_type, method_name, context),
        TypeOld::Array(item_type) => process_array_method_call(item_type, method_name, context),
        TypeOld::TypeRef(struct_info) => {
            if let Some(struct_annotation) = context.get_struct_by_id(struct_info.id) {
                if let Some(method) = struct_annotation.static_methods.get(method_name) {
                    Some(Vasm::simple(method.get_type(), Wat::call_from_stack(&method.wasm_name)))
                } else {
                    None
                }
            } else {
                None
            }
        },
        TypeOld::Struct(struct_info) => {
            if let Some(struct_annotation) = context.get_struct_by_id(struct_info.id) {
                if let Some(method) = struct_annotation.regular_methods.get(method_name) {
                    Some(Vasm::simple(method.get_type(), Wat::call_from_stack(&method.wasm_name)))
                } else {
                    None
                }
            } else {
                None
            }
        },
        TypeOld::Function(_, _) => None,
        TypeOld::Any(_) => None,
    };

    if let Some(method_wasm) = method_info {
        result = process_function_call(Some(method_name), &method_wasm.ty, method_wasm.wat, arguments, access_type, context);
    } else if !parent_type.is_void() {
        context.errors.add(method_name, format!("type `{}` has no method `{}`", parent_type, method_name));
    }

    result
}

pub fn process_function_call(system_method_name: Option<&Identifier>, function_type: &TypeOld, mut function_call: Vec<Wat>, arguments: &ArgumentList, access_type: AccessType, context: &mut ProgramContext) -> Option<Vasm> {
    if let AccessType::Set(set_location) = access_type  {
        context.errors.add(set_location, format!("cannot set result of a function call"));
        return None;
    }

    let (expected_arguments, return_type) = function_type.as_function();

    let mut ok = true;
    let mut source = vec![];
    let mut argument_types = vec![];
    let mut anonymous_types = HashMap::new();

    if arguments.len() != expected_arguments.len() {
        let s = if expected_arguments.len() > 1 { "s" } else { "" };
        context.errors.add(arguments, format!("expected {} argument{}, got {}", expected_arguments.len(), s, arguments.as_vec().len()));
        ok = false;
    }

    for (i, (arg_expr, expected_type)) in arguments.as_vec().iter().zip(expected_arguments.iter()).enumerate() {
        if let Some(arg_wasm) = arg_expr.process(context) {
            argument_types.push(arg_wasm.ty.clone());

            if expected_type.is_assignable_to(&arg_wasm.ty, context, &mut anonymous_types) {
                source.push(arg_wasm);
            } else {
                context.errors.add(arg_expr, format!("argument #{}: expected `{}`, got `{}`", i + 1, expected_type, &arg_wasm.ty));
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

    source.push(Vasm::new(TypeOld::Void, function_call, vec![]));

    match ok {
        true => Some(Vasm::merge(return_type.clone(), source)),
        false => None
    }
}