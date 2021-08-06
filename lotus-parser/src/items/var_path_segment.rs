use std::collections::HashMap;
use parsable::parsable;
use crate::{generation::{ARRAY_GET_FUNC_NAME, Wat}, program::{BuiltinType, ItemType, ProgramContext, Type, Wasm, process_array_field_access, process_array_method_call, process_boolean_field_access, process_boolean_method_call, process_float_field_access, process_float_method_call, process_integer_field_access, process_integer_method_call, process_pointer_field_access, process_pointer_method_call, process_string_field_access, process_string_method_call}};
use super::{ArgumentList, Expression, Identifier};

#[parsable]
pub enum VarPathSegment {
    #[parsable(prefix=".")]
    FieldAccess(Identifier),
    #[parsable(brackets="[]")]
    BracketIndexing(Expression),
    #[parsable(brackets="()", sep=",")]
    FunctionCall(ArgumentList)
}

impl VarPathSegment {
    pub fn is_function_call(&self) -> bool {
        match self {
            VarPathSegment::FunctionCall(_) => true,
            _ => false
        }
    }

    pub fn process(&self, parent_type: &Type, context: &mut ProgramContext) -> Option<Wasm> {
        match self {
            VarPathSegment::FieldAccess(field_name) => process_field_access(parent_type, field_name, context),
            VarPathSegment::BracketIndexing(index_expr) => process_bracket_indexing(parent_type, index_expr, context),
            VarPathSegment::FunctionCall(_) => todo!(),
        }
    }
}

pub fn process_field_access(parent_type: &Type, field_name: &Identifier, context: &mut ProgramContext) -> Option<Wasm> {
    let result = match parent_type {
        Type::Void => None,
        Type::Single(item_type) => match item_type {
            ItemType::Null => None,
            ItemType::Builtin(builtin_type) => match builtin_type {
                BuiltinType::Pointer => process_pointer_field_access(field_name, context),
                BuiltinType::Boolean => process_boolean_field_access(field_name, context),
                BuiltinType::Integer => process_integer_field_access(field_name, context),
                BuiltinType::Float => process_float_field_access(field_name, context),
                BuiltinType::String => process_string_field_access(field_name, context),
            },
            ItemType::Struct(struct_name) => {
                if field_name.is("_") {
                    // special case: `_` refers to the value itself rather than a field
                    // e.g `#foo` means `self.foo`, but `#_` means `self`
                    Some(Wasm::typed(parent_type.clone(), vec![]))
                } else if let Some(struct_annotation) = context.structs.get(struct_name) {
                    if let Some(field) = struct_annotation.fields.get(field_name) {
                        Some(Wasm::typed(
                            field.get_expr_type(),
                            Wat::call(ARRAY_GET_FUNC_NAME, vec![Wat::const_i32(field.offset)])
                        ))
                    } else {
                        None
                    }
                } else {
                    None
                }
            },
            ItemType::Function(_, _) => None,
        },
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
        Type::Single(item_type) => match item_type {
            ItemType::Null => None,
            ItemType::Builtin(builtin_type) => match builtin_type {
                BuiltinType::Pointer => process_pointer_method_call(method_name, context),
                BuiltinType::Boolean => process_boolean_method_call(method_name, context),
                BuiltinType::Integer => process_integer_method_call(method_name, context),
                BuiltinType::Float => process_float_method_call(method_name, context),
                BuiltinType::String => process_string_method_call(method_name, context),
            },
            ItemType::Struct(struct_name) => {
                if let Some(struct_annotation) = context.structs.get(struct_name) {
                    if let Some(method) = struct_annotation.methods.get(method_name) {
                        Some((
                            method.wasm_name.as_str(),
                            method.get_type(),
                        ))
                    } else {
                        None
                    }
                } else {
                    None
                }
            },
            ItemType::Function(_, _) => None,
        },
        Type::Array(item_type) => process_array_method_call(item_type, method_name, context),
        Type::Any(_) => None,

    };

    if let Some((wasm_method_name, method_type)) = method_info {
        let (expected_arguments, return_type) = method_type.as_function();

        if arguments.len() != expected_arguments.len() {
            context.error(arguments, format!("function call arguments: expected {} arguments, got `{}`", expected_arguments.len(), arguments.as_vec().len()));
        }

        let mut ok = true;
        let mut wat = vec![];
        let mut anonymous_types = HashMap::new();

        for (i, (arg_expr, expected_type)) in arguments.as_vec().iter().zip(expected_arguments.iter()).enumerate() {
            if let Some(actual_type_wasm) = arg_expr.process(context) {
                if expected_type.match_actual(&actual_type_wasm.ty, &context.structs, &mut anonymous_types) {
                    wat.extend(actual_type_wasm.wat);
                } else {
                    context.error(arg_expr, format!("function call argument #{}: expected `{}`, got `{}`", i, expected_type, &actual_type_wasm.ty));
                    ok = false;
                }
            }
        }

        match ok {
            true => Some(Wasm::typed(return_type.clone(), wat)),
            false => None
        }
    } else {
        context.error(method_name, format!("type `{}` has no method `{}`", parent_type, method_name));
        None
    }
}

pub fn process_bracket_indexing(parent_type: &Type, index_expr: &Expression, context: &mut ProgramContext) -> Option<Wasm> {
    let mut result = None;
    let mut indexing_ok = false;
    let mut wat = vec![];

    if let Some(index_wasm) = index_expr.process(context) {
        if let Type::Single(ItemType::Builtin(BuiltinType::Integer)) = &index_wasm.ty {
            indexing_ok = true;
        } else {
            context.error(index_expr, format!("bracket indexing argument: expected `{}`, got `{}`", BuiltinType::Integer, &index_wasm.ty));
        }

        wat.extend(index_wasm.wat);
    }

    if let Type::Array(item_type) = parent_type {
        wat.push(Wat::call(ARRAY_GET_FUNC_NAME, vec![]));
        result = Some(Wasm::typed(Box::as_ref(item_type).clone(), wat))
    } else {
        context.error(index_expr, format!("bracket indexing target: expected array, got `{}`", parent_type));
    }

    match indexing_ok {
        true => result,
        false => None
    }
}