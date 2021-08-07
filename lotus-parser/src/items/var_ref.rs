use std::collections::HashMap;

use parsable::parsable;
use crate::{generation::{ARRAY_GET_I32_FUNC_NAME, Wat}, program::{ProgramContext, Type, Wasm, process_array_field_access, process_array_method_call, process_boolean_field_access, process_boolean_method_call, process_float_field_access, process_float_method_call, process_integer_field_access, process_integer_method_call, process_pointer_field_access, process_pointer_method_call, process_string_field_access, process_string_method_call, process_system_field_access, process_system_method_call}};
use super::{ArgumentList, Identifier, VarRefPrefix};

#[parsable]
pub struct VarRef {
    pub name: Identifier,
    pub arguments: Option<ArgumentList>
}

impl VarRef {
    pub fn process_as_field(&self, parent_type: &Type, context: &mut ProgramContext) -> Option<Wasm> {
        match &self.arguments {
            Some(arguments) => process_method_call(parent_type, &self.name, arguments, context),
            None => process_field_access(parent_type, &self.name, context)
        }
    }

    pub fn process_as_variable(&self, context: &mut ProgramContext) -> Option<Wasm> {
        match &self.arguments {
            Some(arguments) => match context.functions.get(&self.name) {
                Some(function_annotation) => process_function_call(function_annotation.wasm_name.as_str(), &function_annotation.get_type(), arguments, context),
                None => {
                    context.error(&self.name, format!("undefined function `{}`", &self.name));
                    None
                },
            },
            None => match context.get_var_info(&self.name) {
                // TODO: handle constants
                Some(var_info) => Some(Wasm::typed(var_info.expr_type.clone(), Wat::get_local(var_info.wasm_name.as_str()))),
                None => {
                    context.error(&self.name, format!("undefined variable `{}`", &self.name));
                    None
                },
            }
        }
    }
}

pub fn process_field_access(parent_type: &Type, field_name: &Identifier, context: &mut ProgramContext) -> Option<Wasm> {
    let result = match parent_type {
        Type::Void => None,
        Type::Null => None,
        Type::System => process_system_field_access(field_name, context),
        Type::Pointer => process_pointer_field_access(field_name, context),
        Type::Boolean => process_boolean_field_access(field_name, context),
        Type::Integer => process_integer_field_access(field_name, context),
        Type::Float => process_float_field_access(field_name, context),
        Type::String => process_string_field_access(field_name, context),
        Type::Struct(struct_name) => {
            if field_name.is("_") {
                // special case: `_` refers to the value itself rather than a field
                // e.g `#foo` means `self.foo`, but `#_` means `self`
                Some(Wasm::typed(parent_type.clone(), vec![]))
            } else if let Some(struct_annotation) = context.structs.get(struct_name) {
                if let Some(field) = struct_annotation.fields.get(field_name) {
                    Some(Wasm::typed(
                        field.get_expr_type(),
                        Wat::call(ARRAY_GET_I32_FUNC_NAME, vec![Wat::const_i32(field.offset)])
                    ))
                } else {
                    None
                }
            } else {
                None
            }
        },
        Type::TypeId(_) => None,
        Type::Function(_, _) => None,
        Type::Array(item_type) => process_array_field_access(item_type, field_name, context),
        Type::Any(_) => None,
        
    };

    if result.is_none() {
        context.error(field_name, format!("type `{}` has no field `{}`", parent_type, field_name));
    }

    result
}

pub fn process_method_call(parent_type: &Type, method_name: &Identifier, arguments: &ArgumentList, context: &mut ProgramContext) -> Option<Wasm> {
    let method_info = match parent_type {
        Type::Void => None,
        Type::Null => None,
        Type::System => process_system_method_call(method_name, context),
        Type::Pointer => process_pointer_method_call(method_name, context),
        Type::Boolean => process_boolean_method_call(method_name, context),
        Type::Integer => process_integer_method_call(method_name, context),
        Type::Float => process_float_method_call(method_name, context),
        Type::String => process_string_method_call(method_name, context),
        Type::Struct(struct_name) => {
            if let Some(struct_annotation) = context.structs.get(struct_name) {
                if let Some(method) = struct_annotation.methods.get(method_name) {
                    Some((
                        method.get_type(),
                        method.wasm_name.as_str(),
                    ))
                } else {
                    None
                }
            } else {
                None
            }
        },
        Type::TypeId(_) => None,
        Type::Function(_, _) => None,
        Type::Array(item_type) => process_array_method_call(item_type, method_name, context),
        Type::Any(_) => None,

    };

    if let Some((method_type, wasm_method_name)) = method_info {
        process_function_call(wasm_method_name, &method_type, arguments, context)
    } else {
        context.error(method_name, format!("type `{}` has no method `{}`", parent_type, method_name));
        None
    }
}

pub fn process_function_call(function_name: &str, function_type: &Type, arguments: &ArgumentList, context: &mut ProgramContext) -> Option<Wasm> {
    let (expected_arguments, return_type) = function_type.as_function();

    if arguments.len() != expected_arguments.len() {
        context.error(arguments, format!("function call arguments: expected {} arguments, got `{}`", expected_arguments.len(), arguments.as_vec().len()));
    }

    let mut ok = true;
    let mut wat = vec![];
    let mut anonymous_types = HashMap::new();

    for (i, (arg_expr, expected_type)) in arguments.as_vec().iter().zip(expected_arguments.iter()).enumerate() {
        if let Some(actual_type_wasm) = arg_expr.process(context) {
            if expected_type.is_assignable(&actual_type_wasm.ty, &context.structs, &mut anonymous_types) {
                wat.extend(actual_type_wasm.wat);
            } else {
                context.error(arg_expr, format!("function call argument #{}: expected `{}`, got `{}`", i, expected_type, &actual_type_wasm.ty));
                ok = false;
            }
        }
    }

    wat.push(Wat::call(function_name, vec![]));

    match ok {
        true => Some(Wasm::typed(return_type.clone(), wat)),
        false => None
    }
}