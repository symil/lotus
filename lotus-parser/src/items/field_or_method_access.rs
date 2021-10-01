use std::{cell::Ref, collections::HashMap, rc::Rc};
use indexmap::IndexMap;
use parsable::parsable;
use colored::*;
use crate::{program::{AccessType, FieldKind, FunctionBlueprint, ParameterTypeInfo, ProgramContext, Type, VI, VariableKind, Vasm}, utils::Link, vasm};
use super::{ArgumentList, Identifier, VarPrefix};

#[parsable]
pub struct FieldOrMethodAccess {
    pub name: Identifier,
    pub arguments: Option<ArgumentList>
}

impl FieldOrMethodAccess {
    pub fn has_side_effects(&self) -> bool {
        self.arguments.is_some()
    }

    pub fn process(&self, parent_type: &Type, field_kind: FieldKind, type_hint: Option<&Type>, access_type: AccessType, context: &mut ProgramContext) -> Option<Vasm> {
        match &self.arguments {
            Some(arguments) => process_method_call(parent_type, field_kind, &self.name, &[], arguments, type_hint, access_type, context),
            None => process_field_access(parent_type, &self.name, access_type, context)
        }
    }
}

pub fn process_field_access(parent_type: &Type, field_name: &Identifier, access_type: AccessType, context: &mut ProgramContext) -> Option<Vasm> {
    let mut result = None;

    if let Some(field_info) = parent_type.get_field(field_name.as_str()) {
        let field_type = field_info.ty.replace_generics(Some(parent_type), &[]);
        let instruction = match access_type {
            AccessType::Get => VI::get_field(&field_type, field_info.offset),
            AccessType::Set(_) => VI::set_field_from_stack(&field_type, field_info.offset),
        };

        result = Some(Vasm::new(field_type, vec![], vec![instruction]));
    } else {
        context.errors.add(field_name, format!("type `{}` has no field `{}`", parent_type, field_name));
    }

    result
}

pub fn process_method_call(caller_type: &Type, field_kind: FieldKind, method_name: &Identifier, parameters: &[Type], arguments: &ArgumentList, type_hint: Option<&Type>, access_type: AccessType, context: &mut ProgramContext) -> Option<Vasm> {
    let mut result = None;

    if let Some(function_blueprint) = caller_type.get_method(field_kind, method_name.as_str(), context) {
        result = process_function_call(Some(caller_type), method_name, function_blueprint, parameters, arguments, type_hint, access_type, context);
    } else if !caller_type.is_undefined() {
        context.errors.add(method_name, format!("type `{}` has no {}method `{}`", caller_type, field_kind.get_qualifier(), method_name.as_str().bold()));
    }

    result
}

pub fn process_function_call(caller_type: Option<&Type>, function_name: &Identifier, function_wrapped: Link<FunctionBlueprint>, parameters: &[Type], arguments: &ArgumentList, type_hint: Option<&Type>, access_type: AccessType, context: &mut ProgramContext) -> Option<Vasm> {
    if let AccessType::Set(set_location) = access_type  {
        context.errors.add(set_location, format!("cannot set result of a function call"));
        return None;
    }

    let mut result = Vasm::empty();
    let mut final_function_parameters = vec![];

    let (function_name, return_type) = function_wrapped.with_ref(|function_unwrapped| {
        let mut return_type = None;
        let expected_arg_count = function_unwrapped.arguments.len();

        if arguments.len() != expected_arg_count {
            let s = if expected_arg_count > 1 { "s" } else { "" };

            context.errors.add(arguments, format!("expected {} argument{}, got {}", expected_arg_count, s, arguments.as_vec().len()));
        } else {
            let arg_vasms : Vec<Vasm> = arguments.as_vec().iter().enumerate().map(|(i, arg)| {
                let arg_type = &function_unwrapped.arguments[i].ty;
                let hint = match arg_type.contains_function_parameter() {
                    true => None,
                    false => Some(arg_type),
                };

                arg.process(hint, context).unwrap_or_default()
            }).collect();
            let arg_types : Vec<&Type> = arg_vasms.iter().map(|vasm| &vasm.ty).collect();

            match infer_function_parameters(function_name, &function_unwrapped, &arg_types, type_hint, context) {
                Some(parameters) => {
                    for (expected_param, actual_param) in function_unwrapped.parameters.values().zip(parameters.iter()) {
                        actual_param.check_match_interface_list(&expected_param.required_interfaces, function_name, context);
                    }

                    for (i, (expected_arg, arg_vasm)) in function_unwrapped.arguments.iter().zip(arg_vasms.into_iter()).enumerate() {
                        let expected_type = expected_arg.ty.replace_generics(caller_type, &parameters);

                        if arg_vasm.ty.is_assignable_to(&expected_type) {
                            result.extend(arg_vasm);
                        } else if !arg_vasm.ty.is_undefined() {
                            context.errors.add(&arguments.as_vec()[i], format!("argument #{}: expected `{}`, got `{}`", i + 1, &expected_type, &arg_vasm.ty));
                        }
                    }

                    final_function_parameters = parameters;
                    return_type = function_unwrapped.return_value.as_ref().and_then(|var_info| Some(var_info.ty.replace_generics(caller_type, &final_function_parameters)));
                },
                None => {}
            }
        }

        (
            function_unwrapped.name.to_string(),
            return_type.unwrap_or(Type::Undefined)
        )
    });

    let call_instruction = match caller_type {
        Some(ty) => VI::call_method(ty, function_wrapped.clone(), &final_function_parameters, vasm![]),
        None => VI::call_function(function_wrapped.clone(), &final_function_parameters, vasm![]),
    };

    result.extend(Vasm::new(return_type, vec![], vec![call_instruction]));

    Some(result)
}

fn infer_function_parameters(function_name: &Identifier, function_unwrapped: &FunctionBlueprint, arg_types: &[&Type], type_hint: Option<&Type>, context: &mut ProgramContext) -> Option<Vec<Type>> {
    let mut result = vec![];

    for (i, parameter) in function_unwrapped.parameters.values().enumerate() {
        let mut ok = false;

        if let Some(hint_return_type) = type_hint {
            if let Some(return_value) = &function_unwrapped.return_value {
                let expected_return_type = &return_value.ty;

                if expected_return_type.is_function_parameter(parameter) {
                    result.push(hint_return_type.clone());
                    ok = true;
                }
            }
        }

        if !ok {
            for (expected_arg_var, actual_arg_type) in function_unwrapped.arguments.iter().zip(arg_types.iter()) {
                let expected_arg_type = &expected_arg_var.ty;

                if expected_arg_type.is_function_parameter(parameter) {
                    result.push((*actual_arg_type).clone());
                    ok = true;
                    break;
                }
            }
        }

        if !ok {
            context.errors.add(function_name, format!("`{}`: cannot infer type parameter #{}", function_name.as_str().bold(), i + 1));
            return None;
        }
    }

    Some(result)
}