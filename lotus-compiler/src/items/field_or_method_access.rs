use std::{cell::Ref, collections::{HashMap, HashSet}, rc::Rc};
use indexmap::IndexMap;
use parsable::parsable;
use colored::*;
use crate::{program::{AccessType, AnonymousFunctionCallDetails, DUPLICATE_INT_WASM_FUNC_NAME, FieldKind, FunctionBlueprint, FunctionCall, GET_AT_INDEX_FUNC_NAME, NONE_LITERAL, NONE_METHOD_NAME, NamedFunctionCallDetails, ParameterTypeInfo, ProgramContext, Type, VI, VariableInfo, VariableKind, Vasm, Wat, print_type_list, print_type_ref_list}, utils::Link, vasm, wat};
use super::{ArgumentList, Identifier, IdentifierWrapper, VarPrefix};

#[parsable]
pub struct FieldOrMethodAccess {
    pub name: IdentifierWrapper,
    pub arguments: Option<ArgumentList>
}

impl FieldOrMethodAccess {
    pub fn has_side_effects(&self) -> bool {
        true
    }

    pub fn process(&self, parent_type: &Type, field_kind: FieldKind, type_hint: Option<&Type>, access_type: AccessType, context: &mut ProgramContext) -> Option<Vasm> {
        match self.name.process(context) {
            Some(name) => match &self.arguments {
                Some(arguments) => process_method_call(parent_type, field_kind, &name, &[], arguments, type_hint, access_type, context),
                None => process_field_access(parent_type, field_kind, &name, access_type, context)
            },
            None => None,
        }
    }
}

pub fn process_field_access(parent_type: &Type, field_kind: FieldKind, field_name: &Identifier, access_type: AccessType, context: &mut ProgramContext) -> Option<Vasm> {
    let mut result = None;

    match field_kind {
        FieldKind::Regular => {
            if let Some(field_info) = parent_type.get_field(field_name.as_str()) {
                let field_type = field_info.ty.replace_parameters(Some(parent_type), &[]);
                let instruction = match access_type {
                    AccessType::Get => VI::get_field(&field_type, field_info.offset),
                    AccessType::Set(location) => VI::set_field(&field_type, field_info.offset, vasm![VI::placeholder(location)]),
                };

                context.access_shared_identifier(&field_info.name, field_name);

                result = Some(Vasm::new(field_type, vec![], vec![instruction]));
            } else if !parent_type.is_undefined() {
                context.errors.add_generic(field_name, format!("type `{}` has no field `{}`", parent_type, field_name.as_str().bold()));
            }
        },
        FieldKind::Static => {
            match field_name.as_str() == NONE_LITERAL {
                true => {
                    result = Some(Vasm::new(parent_type.clone(), vec![], vec![VI::call_static_method(parent_type, NONE_METHOD_NAME, &[], vec![], context)]));
                },
                false => match parent_type {
                    Type::Actual(info) => info.type_blueprint.with_ref(|type_unwrapped| {
                        if let Some(variant_info) = type_unwrapped.enum_variants.get(field_name.as_str()) {
                            context.access_shared_identifier(&variant_info.name, field_name);

                            match access_type {
                                AccessType::Get => {
                                    result = Some(Vasm::new(parent_type.clone(), vec![], vec![VI::int(variant_info.value)]));
                                },
                                AccessType::Set(location) => {
                                    context.errors.add_generic(location, format!("cannot set value of enum variant"));
                                },
                            }
                        }
                    }),
                    _ => {}
                }
            }

            if result.is_none() {
                context.errors.add_generic(field_name, format!("type `{}` has no enum variant `{}`", parent_type, field_name.as_str().bold()));
            }
        }
    };

    result
}

pub fn process_method_call(caller_type: &Type, field_kind: FieldKind, method_name: &Identifier, parameters: &[Type], arguments: &ArgumentList, type_hint: Option<&Type>, access_type: AccessType, context: &mut ProgramContext) -> Option<Vasm> {
    let mut result = None;

    if let Some(func_ref) = caller_type.get_method(field_kind, method_name.as_str(), context) {
        let caller_type = func_ref.this_type.replace_parameters(Some(caller_type), &[]);
        let function_call = func_ref.function.with_ref(|function_unwrapped| {
            match function_unwrapped.get_dynamic_index() {
                Some(dynamic_index) => FunctionCall::Anonymous(AnonymousFunctionCallDetails {
                    signature: function_unwrapped.signature.replace_parameters(Some(&caller_type), &[]),
                    function_offset: dynamic_index,
                }),
                None => FunctionCall::Named(NamedFunctionCallDetails {
                    caller_type: Some(caller_type),
                    function: func_ref.function.clone(),
                    parameters: parameters.to_vec(),
                }),
            }
        });

        result = process_function_call(method_name, function_call, arguments, type_hint, access_type, context);
    } else if !caller_type.is_undefined() {
        context.errors.add_generic(method_name, format!("type `{}` has no {}method `{}`", caller_type, field_kind.get_qualifier(), method_name.as_str().bold()));
    }

    result
}

pub fn process_function_call(function_name: &Identifier, mut function_call: FunctionCall, arguments: &ArgumentList, type_hint: Option<&Type>, access_type: AccessType, context: &mut ProgramContext) -> Option<Vasm> {
    if let FunctionCall::Named(details) = &function_call {
        context.access_wrapped_shared_identifier(&details.function, function_name);
    }

    if let AccessType::Set(set_location) = access_type  {
        context.errors.add_generic(set_location, format!("cannot set result of a function call"));
        return None;
    }

    let (signature, caller_type) = match &function_call {
        FunctionCall::Named(details) => (details.function.borrow().signature.clone(), details.caller_type.clone()),
        FunctionCall::Anonymous(details) => (details.signature.clone(), details.signature.this_type.clone()),
    };
    let is_var_call = match &function_call {
        FunctionCall::Named(_) => false,
        FunctionCall::Anonymous(details) => details.signature.this_type.is_none(),
    };

    let mut function_parameters = match &function_call {
        FunctionCall::Named(details) => details.function.borrow().parameters.values().map(|info| Type::FunctionParameter(info.clone())).collect(),
        FunctionCall::Anonymous(_) => vec![],
    };

    let mut remaining_param_indexes_to_infer : HashSet<usize> = HashSet::new();

    for i in 0..function_parameters.len() {
        remaining_param_indexes_to_infer.insert(i);
    }

    if let Some(ty) = type_hint {
        function_parameters = infer_function_parameters(&function_parameters, &mut remaining_param_indexes_to_infer, ty, &signature.return_type);
    }

    let arg_vasms : Vec<Vasm> = arguments.as_vec().iter().enumerate().map(|(i, arg)| {
        let hint = match signature.argument_types.get(i) {
            Some(ty) => match is_var_call {
                true => Some(ty.clone()),
                false => Some(ty.replace_parameters(caller_type.as_ref(), &function_parameters)),
            },
            None => None,
        };

        match arg.process(hint.as_ref(), context) {
            Some(vasm) => {
                if let Some(expected_type) = signature.argument_types.get(i) {
                    function_parameters = infer_function_parameters(&function_parameters, &mut remaining_param_indexes_to_infer, &vasm.ty, expected_type);
                }

                vasm
            },
            None => {
                Vasm::new(Type::Undefined, vec![], vec![])
            }
        }
    }).collect();

    for i in remaining_param_indexes_to_infer.into_iter() {
        context.errors.add_generic(function_name, format!("`{}`: cannot infer type parameter `{}`", function_name.as_str().bold(), function_parameters[i]));
    }

    if let FunctionCall::Named(details) = &mut function_call {
        details.function.with_ref(|function_unwrapped| {
            for (expected_param, actual_param) in function_unwrapped.parameters.values().zip(function_parameters.iter()) {
                actual_param.check_match_interface_list(&expected_param.required_interfaces, function_name, context);
            }
        });
    }

    match arguments.len() == signature.argument_types.len() {
        true => {
            for (i, (arg_type, arg_vasm)) in signature.argument_types.iter().zip(arg_vasms.iter()).enumerate() {
                let expected_type = match is_var_call {
                    true => arg_type.clone(),
                    false => arg_type.replace_parameters(caller_type.as_ref(), &function_parameters),
                };

                if !arg_vasm.ty.is_undefined() {
                    if arg_vasm.ty.is_ambiguous() {
                        context.errors.add_generic(&arguments.as_vec()[i], format!("cannot infer type"));
                    } else if !arg_vasm.ty.is_assignable_to(&expected_type) {
                        context.errors.add_generic(&arguments.as_vec()[i], format!("expected `{}`, got `{}`", &expected_type, &arg_vasm.ty));
                    }
                }
            }
        },
        false => {
            let s = if signature.argument_types.len() > 1 { "s" } else { "" };

            context.errors.add_generic(arguments, format!("expected {} argument{}, got {}", signature.argument_types.len(), s, arguments.len()));
        }
    };

    let return_type = match is_var_call {
        true => signature.return_type.clone(),
        false => signature.return_type.replace_parameters(caller_type.as_ref(), &function_parameters),
    };

    if let FunctionCall::Named(details) = &mut function_call {
        details.parameters = function_parameters;
    }

    Some(Vasm::new(return_type, vec![], vec![
        VI::call_function(function_call, Vasm::merge(arg_vasms))
    ]))
}

fn infer_function_parameters(function_parameters: &[Type], remaining_type_indexes_to_infer: &mut HashSet<usize>, actual_type: &Type, expected_type: &Type) -> Vec<Type> {
    let mut result = vec![];

    for (i, function_parameter) in function_parameters.iter().enumerate() {
        if let Type::FunctionParameter(info) = function_parameter {
            if let Some(ty) = expected_type.infer_function_parameter(info, actual_type) {
                result.push(ty);
                remaining_type_indexes_to_infer.remove(&i);
                continue;
            }
        }

        result.push(function_parameter.clone());
    }

    result
}