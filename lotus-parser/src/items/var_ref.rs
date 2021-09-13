use std::{cell::Ref, collections::HashMap};

use parsable::parsable;
use crate::{program::{AccessType, FunctionBlueprint, PTR_GET_METHOD_NAME, PTR_SET_METHOD_NAME, ProgramContext, Type, VI, VariableKind, Vasm}, utils::Link};
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
                    process_function_call(function_blueprint, arguments, access_type, context)
                },
                None => {
                    context.errors.add(&self.name, format!("undefined function `{}`", &self.name));
                    None
                },
            },
            None => match context.get_var_info(&self.name) {
                Some(var_info) => match access_type {
                    AccessType::Get => Some(Vasm::new(var_info.ty.clone(), vec![], vec![VI::get(var_info)])),
                    AccessType::Set(_) => Some(Vasm::new(var_info.ty.clone(), vec![], vec![VI::set_from_stack(var_info)])),
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

pub fn process_field_access(parent_type: &Type, field_name: &Identifier, access_type: AccessType, context: &mut ProgramContext) -> Option<Vasm> {
    let mut result = None;

    if let Some(field_details) = parent_type.get_field(field_name.as_str()) {
        let method_name = match access_type {
            AccessType::Get => PTR_GET_METHOD_NAME,
            AccessType::Set(_) => PTR_SET_METHOD_NAME,
        };

        result = Some(Vasm::new(field_details.ty.clone(), vec![], vec![
            VI::call_function(parent_type.get_method(method_name), vec![VI::int(field_details.offset)])
        ]));
    } else {
        context.errors.add(field_name, format!("type `{}` has no field `{}`", parent_type, field_name));
    }

    result
}

pub fn process_method_call(parent_type: &Type, method_name: &Identifier, arguments: &ArgumentList, access_type: AccessType, context: &mut ProgramContext) -> Option<Vasm> {
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
                    Some(Vasm::simple(method.get_type(), Wat::call_from_stack(&method.vasm_name)))
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
                    Some(Vasm::simple(method.get_type(), Wat::call_from_stack(&method.vasm_name)))
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

    if let Some(method_vasm) = method_info {
        result = process_function_call(Some(method_name), &method_vasm.ty, method_vasm.wat, arguments, access_type, context);
    } else if !parent_type.is_void() {
        context.errors.add(method_name, format!("type `{}` has no method `{}`", parent_type, method_name));
    }

    result
}

pub fn process_function_call(function_blueprint: &Link<FunctionBlueprint>, arguments: &ArgumentList, access_type: AccessType, context: &mut ProgramContext) -> Option<Vasm> {
    if let AccessType::Set(set_location) = access_type  {
        context.errors.add(set_location, format!("cannot set result of a function call"));
        return None;
    }

    let function_content = function_blueprint.borrow();
    let expected_arg_count = function_content.arguments.len();
    let mut result = Vasm::empty();

    if arguments.len() != expected_arg_count {
        let s = if expected_arg_count > 1 { "s" } else { "" };

        context.errors.add(arguments, format!("expected {} argument{}, got {}", expected_arg_count, s, arguments.as_vec().len()));
    }

    for (i, arg_expr) in arguments.as_vec().iter().enumerate() {
        if let Some(arg_vasm) = arg_expr.process(context) {
            if i < expected_arg_count {
                let expected_type = &function_content.arguments[0].ty;

                if expected_type.is_assignable_to(&arg_vasm.ty) {
                    result.extend(arg_vasm);
                } else {
                    context.errors.add(arg_expr, format!("argument #{}: expected `{}`, got `{}`", i + 1, expected_type, &arg_vasm.ty));
                }
            }
        }
    }

    let return_type = function_content.return_value.and_then(|var_info| Some(var_info.ty.clone())).unwrap_or(Type::Void);

    result.extend(Vasm::new(return_type, vec![], vec![VI::call_function_from_stack(function_blueprint)]));

    Some(result)
}