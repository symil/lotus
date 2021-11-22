use std::{cell::Ref, collections::HashMap, rc::Rc};
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
                    AccessType::Set(_) => VI::set_field_from_stack(&field_type, field_info.offset),
                };

                result = Some(Vasm::new(field_type, vec![], vec![instruction]));
            } else if !parent_type.is_undefined() {
                context.errors.add(field_name, format!("type `{}` has no field `{}`", parent_type, field_name.as_str().bold()));
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
                            match access_type {
                                AccessType::Get => {
                                    result = Some(Vasm::new(parent_type.clone(), vec![], vec![VI::int(variant_info.value)]));
                                },
                                AccessType::Set(location) => {
                                    context.errors.add(location, format!("cannot set value of enum variant"));
                                },
                            }
                        }
                    }),
                    _ => {}
                }
            }

            if result.is_none() {
                context.errors.add(field_name, format!("type `{}` has no enum variant `{}`", parent_type, field_name.as_str().bold()));
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
        context.errors.add(method_name, format!("type `{}` has no {}method `{}`", caller_type, field_kind.get_qualifier(), method_name.as_str().bold()));
    }

    result
}

pub fn process_function_call(function_name: &Identifier, mut function_call: FunctionCall, arguments: &ArgumentList, type_hint: Option<&Type>, access_type: AccessType, context: &mut ProgramContext) -> Option<Vasm> {
    if let AccessType::Set(set_location) = access_type  {
        context.errors.add(set_location, format!("cannot set result of a function call"));
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

    let arg_vasms : Vec<Vasm> = arguments.as_vec().iter().enumerate().map(|(i, arg)| {
        let hint = match signature.argument_types.get(i) {
            Some(ty) => match is_var_call {
                true => Some(ty.clone()),
                false => Some(ty.replace_parameters(caller_type.as_ref(), &[])),
            },
            None => None,
        };

        arg.process(hint.as_ref(), context).unwrap_or_else(|| Vasm::new(Type::Undefined, vec![], vec![]))
    }).collect();
    let arg_types : Vec<&Type> = arg_vasms.iter().map(|vasm| &vasm.ty).collect();

    if let FunctionCall::Named(details) = &mut function_call {
        if let Some(parameters) = infer_function_parameters(function_name, &details.function, &arg_types, type_hint, context) {
            details.function.with_ref(|function_unwrapped| {
                for (expected_param, actual_param) in function_unwrapped.parameters.values().zip(parameters.iter()) {
                    actual_param.check_match_interface_list(&expected_param.required_interfaces, function_name, context);
                }
            });

            // if function_name.as_str() == "map" {
            //     print_type_ref_list(&arg_types);
            //     print_type_list(&parameters);
            // }

            details.parameters = parameters;
        }
    }

    let parameters = match &function_call {
        FunctionCall::Named(details) => details.parameters.as_slice(),
        FunctionCall::Anonymous(_) => &[],
    };

    match arguments.len() == signature.argument_types.len() {
        true => {
            for (i, (arg_type, arg_vasm)) in signature.argument_types.iter().zip(arg_vasms.iter()).enumerate() {
                let expected_type = match is_var_call {
                    true => arg_type.clone(),
                    false => arg_type.replace_parameters(caller_type.as_ref(), parameters),
                };

                if !arg_vasm.ty.is_undefined() {
                    if arg_vasm.ty.is_ambiguous() {
                        context.errors.add(&arguments.as_vec()[i], format!("cannot infer type"));
                    } else if !arg_vasm.ty.is_assignable_to(&expected_type) {
                        context.errors.add(&arguments.as_vec()[i], format!("expected `{}`, got `{}`", &expected_type, &arg_vasm.ty));
                    }
                }
            }
        },
        false => {
            let s = if signature.argument_types.len() > 1 { "s" } else { "" };

            context.errors.add(arguments, format!("expected {} argument{}, got {}", signature.argument_types.len(), s, arguments.len()));
        }
    };

    let return_type = match is_var_call {
        true => signature.return_type.clone(),
        false => signature.return_type.replace_parameters(caller_type.as_ref(), parameters),
    };

    Some(Vasm::new(return_type, vec![], vec![
        VI::call_function(function_call, Vasm::merge(arg_vasms))
    ]))
}

fn infer_function_parameters(function_name: &Identifier, function_wrapped: &Link<FunctionBlueprint>, arg_types: &[&Type], type_hint: Option<&Type>, context: &mut ProgramContext) -> Option<Vec<Type>> {
    function_wrapped.with_ref(|function_unwrapped| {
        let mut result = vec![];

        for (i, parameter) in function_unwrapped.parameters.values().enumerate() {
            let mut ok = false;

            if let Some(hint_return_type) = type_hint {
                let expected_return_type = &function_unwrapped.signature.return_type;

                if let Some(inferred_type) = expected_return_type.infer_function_parameter(parameter, hint_return_type) {
                    result.push(inferred_type);
                    ok = true;
                }
            }

            if !ok {
                for (expected_arg_type, actual_arg_type) in function_unwrapped.signature.argument_types.iter().zip(arg_types.iter()) {
                    if let Some(inferred_type) = expected_arg_type.infer_function_parameter(parameter, *actual_arg_type) {
                        result.push(inferred_type);
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
    })
}