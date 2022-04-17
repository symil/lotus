use std::{cell::Ref, collections::{HashMap, HashSet}, rc::Rc};
use indexmap::IndexMap;
use parsable::parsable;
use colored::*;
use crate::{program::{AccessType, AnonymousFunctionCallDetails, DUPLICATE_INT_WASM_FUNC_NAME, FieldKind, FunctionBlueprint, FunctionCall, GET_AT_INDEX_FUNC_NAME, NONE_LITERAL, NONE_METHOD_NAME, NamedFunctionCallDetails, ParameterTypeInfo, ProgramContext, Type, VariableInfo, VariableKind, Vasm, Wat, print_type_list, print_type_ref_list, TypeContent}, utils::Link, wat, language_server::FieldCompletionOptions};
use super::{ParsedArgumentList, Identifier, ParsedIdentifierWrapper, ParsedVarPrefixToken, ParsedDotToken};

#[parsable]
pub struct ParsedFieldOrMethodAccess {
    pub dot: ParsedDotToken,
    pub name: Option<ParsedIdentifierWrapper>,
    pub arguments: Option<ParsedArgumentList>
}

impl ParsedFieldOrMethodAccess {
    pub fn has_side_effects(&self) -> bool {
        true
    }

    pub fn process(&self, parent_type: &Type, field_kind: FieldKind, type_hint: Option<&Type>, access_type: AccessType, context: &mut ProgramContext) -> Option<Vasm> {
        context.completion_provider.add_field_completion(&self.dot, parent_type, type_hint, Some(&FieldCompletionOptions {
            show_methods: true,
            insert_arguments: self.arguments.is_none(),
            ..Default::default()
        }));

        match &self.name {
            Some(identifier) => {
                match identifier.process(context) {
                    Some(name) => {
                        context.completion_provider.add_field_completion(&name.location, parent_type, type_hint, Some(&FieldCompletionOptions {
                            show_methods: true,
                            insert_arguments: self.arguments.is_none(),
                            ..Default::default()
                        }));

                        match &self.arguments {
                            Some(arguments) => process_method_call(parent_type, field_kind, &name, &[], arguments, type_hint, access_type, context),
                            None => process_field_access(parent_type, field_kind, &name, access_type, context)
                        }
                    },
                    None => {
                        
                        None
                    },
                }
            },
            None => {
                context.errors.generic(self, format!("expected field or method name"));
                None
            },
        }
    }
}

pub fn process_field_access(parent_type: &Type, field_kind: FieldKind, field_name: &Identifier, access_type: AccessType, context: &mut ProgramContext) -> Option<Vasm> {
    let mut result = None;

    match field_kind {
        FieldKind::Regular => {
            if let Some(field_info) = parent_type.get_field(field_name.as_str()) {
                let field_type = field_info.ty.replace_parameters(Some(parent_type), &[]);
                let check_location = match context.root_tags.check_field_access {
                    true => Some(&field_name.location),
                    false => None,
                };
                let mut vasm = context.vasm()
                    .set_type(&field_type);

                match access_type {
                    AccessType::Get => {
                        vasm = vasm.get_field(&field_type, field_info.offset, check_location);
                    },
                    AccessType::Set(location) => {
                        vasm = vasm.set_field(&field_type, field_info.offset, check_location, context.vasm().placeholder(location));
                    },
                };

                context.rename_provider.add_occurence(field_name, &field_info.name);
                context.definition_provider.set_definition(field_name, &field_info.name);
                context.hover_provider.set_type(field_name, &field_type);

                result = Some(vasm);
            } else if !parent_type.is_undefined() {
                context.errors.generic(field_name, format!("type `{}` has no field `{}`", parent_type, field_name.as_str().bold()));
            }
        },
        FieldKind::Static => {
            match field_name.as_str() == NONE_LITERAL {
                true => {
                    result = Some(context.vasm()
                        .call_static_method(parent_type, NONE_METHOD_NAME, &[], vec![], context)
                        .set_type(parent_type)
                    );
                },
                false => match parent_type.content() {
                    TypeContent::Actual(info) => info.type_blueprint.with_ref(|type_unwrapped| {
                        if let Some(variant_info) = type_unwrapped.enum_variants.get(field_name.as_str()) {
                            context.rename_provider.add_occurence(field_name, &variant_info.name);
                            context.definition_provider.set_definition(field_name, &variant_info.name);
                            context.hover_provider.set_type(field_name, parent_type);

                            match access_type {
                                AccessType::Get => {
                                    result = Some(context.vasm()
                                        .int(variant_info.value)
                                        .set_type(parent_type)
                                    );
                                },
                                AccessType::Set(location) => {
                                    context.errors.generic(location, format!("cannot set value of enum variant"));
                                },
                            }
                        }
                    }),
                    _ => {}
                }
            }

            if result.is_none() {
                context.errors.generic(field_name, format!("type `{}` has no enum variant `{}`", parent_type, field_name.as_str().bold()));
            }
        }
    };

    result
}

pub fn process_method_call(caller_type: &Type, field_kind: FieldKind, method_name: &Identifier, parameters: &[Type], arguments: &ParsedArgumentList, type_hint: Option<&Type>, access_type: AccessType, context: &mut ProgramContext) -> Option<Vasm> {
    let mut result = None;

    if let Some(func_ref) = caller_type.get_method(field_kind, method_name.as_str(), context) {
        let caller_type = func_ref.this_type.replace_parameters(Some(caller_type), &[]);
        let function_call = func_ref.function.with_ref(|function_unwrapped| {
            match function_unwrapped.get_dynamic_index() {
                Some(dynamic_index) => {
                    context.rename_provider.add_occurence(method_name, &function_unwrapped.name);
                    context.definition_provider.set_definition(method_name, &function_unwrapped.name);
                    context.hover_provider.set_type(method_name, &function_unwrapped.get_self_type());

                    FunctionCall::Anonymous(AnonymousFunctionCallDetails {
                        signature: function_unwrapped.signature.replace_parameters(Some(&caller_type), &[]),
                        function_offset: dynamic_index,
                    })
                },
                None => FunctionCall::Named(NamedFunctionCallDetails {
                    caller_type: Some(caller_type),
                    function: func_ref.function.clone(),
                    parameters: parameters.to_vec(),
                }),
            }
        });

        result = process_function_call(Some(method_name), function_call, arguments, type_hint, access_type, context);
    } else if !caller_type.is_undefined() {
        context.errors.generic(method_name, format!("type `{}` has no {}method `{}`", caller_type, field_kind.get_qualifier(), method_name.as_str().bold()));
    }

    result
}

pub fn process_function_call(function_identifier: Option<&Identifier>, mut function_call: FunctionCall, arguments: &ParsedArgumentList, type_hint: Option<&Type>, access_type: AccessType, context: &mut ProgramContext) -> Option<Vasm> {
    let function_name = match &function_identifier {
        Some(identifier) => identifier.as_str(),
        None => "",
    };

    if let Some(identifier) = function_identifier {
        if let FunctionCall::Named(details) = &function_call {
            context.rename_provider.add_occurence(identifier, &details.function.borrow().name);
            context.definition_provider.set_definition(identifier, &details.function.borrow().name);
            context.hover_provider.set_type(identifier, &details.function.borrow().get_self_type());
        }
    }

    if let AccessType::Set(set_location) = access_type  {
        context.errors.generic(set_location, format!("cannot set result of a function call"));
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
        FunctionCall::Named(details) => details.function.borrow().parameters.values().map(|info| Type::function_parameter(&info)).collect(),
        FunctionCall::Anonymous(_) => vec![],
    };

    let mut remaining_param_indexes_to_infer : HashSet<usize> = HashSet::new();

    for i in 0..function_parameters.len() {
        remaining_param_indexes_to_infer.insert(i);
    }

    if let Some(ty) = type_hint {
        function_parameters = infer_function_parameters(&function_parameters, &mut remaining_param_indexes_to_infer, ty, &signature.return_type);
    }

    context.signature_help_provider.declare_signature(&arguments.location, function_name, &function_call);

    for i in 0..arguments.len().max(1).min(signature.argument_types.len()) {
        context.signature_help_provider.add_argument_location(&arguments.location, i, arguments.get_location_including_separator(i + 1));
    }

    let mut arg_vasms : Vec<Vasm> = arguments.into_iter().enumerate().map(|(i, arg)| {
        let hint = match signature.argument_types.get(i) {
            Some(ty) => match is_var_call {
                true => Some(ty.clone()),
                false => Some(ty.replace_parameters(caller_type.as_ref(), &function_parameters)),
            },
            None => None,
        };
        let mut arg_vasm = context.vasm();

        if let Some(arg_expression) = arg {
            if let Some(vasm) = arg_expression.process(hint.as_ref(), context) {
                if let Some(expected_type) = signature.argument_types.get(i) {
                    function_parameters = infer_function_parameters(&function_parameters, &mut remaining_param_indexes_to_infer, &vasm.ty, expected_type);
                }

                arg_vasm = vasm;
            }
        } else {
            context.errors.expected_argument(&arguments.get_location_including_separator(i).unwrap().set_start_with_offset(1));
        }

        arg_vasm
    }).collect();

    if let FunctionCall::Named(details) = &function_call {
        details.function.with_ref(|function_unwrapped| {
            for i in arg_vasms.len()..function_unwrapped.arguments.len() {
                let arg = &function_unwrapped.arguments[i];

                if arg.is_optional {
                    arg_vasms.push(arg.default_value.clone());
                }
            }
        });
    }

    for i in remaining_param_indexes_to_infer.into_iter() {
        if function_name == "todo" {
            // TODO: improve default type parameters
            function_parameters[i] = context.void_type();
        } else {
            context.errors.generic(function_identifier.as_ref().unwrap(), format!("cannot infer type parameter `{}`", function_parameters[i]));
        }
    }

    if let FunctionCall::Named(details) = &mut function_call {
        details.function.with_ref(|function_unwrapped| {
            for (expected_param, actual_param) in function_unwrapped.parameters.values().zip(function_parameters.iter()) {
                let location = function_identifier.as_ref().unwrap();

                actual_param.check_match_param(expected_param, location, context);
            }
        });
    }

    match arg_vasms.len() == signature.argument_types.len() {
        true => {
            for (i, (arg_type, arg_vasm)) in signature.argument_types.iter().zip(arg_vasms.iter()).enumerate() {
                let expected_type = match is_var_call {
                    true => arg_type.clone(),
                    false => arg_type.replace_parameters(caller_type.as_ref(), &function_parameters),
                };

                if let Some(location) = &arguments.get_location(i) {
                    if !arg_vasm.ty.is_undefined() {
                        if arg_vasm.ty.is_ambiguous() {
                            context.errors.generic(location, format!("cannot infer type"));
                        } else if !arg_vasm.ty.is_assignable_to(&expected_type) {
                            context.errors.type_mismatch(location, &expected_type, &arg_vasm.ty);
                        }
                    }
                }
            }
        },
        false => {
            let s = if signature.argument_types.len() > 1 { "s" } else { "" };

            context.errors.generic(arguments, format!("expected {} argument{}, got {}", signature.argument_types.len(), s, arguments.len()));
        }
    };

    let return_type = match is_var_call {
        true => signature.return_type.clone(),
        false => signature.return_type.replace_parameters(caller_type.as_ref(), &function_parameters),
    };

    let mut check_location = None;

    if context.root_tags.check_field_access {
        if signature.this_type.is_some() {
            if let Some(identifier) = function_identifier {
                check_location = Some(&identifier.location)
            }
        }
    }

    let result = match function_call {
        FunctionCall::Named(details) => {
            context.vasm()
                .call_function_named(check_location, details.caller_type.as_ref(), &details.function, &function_parameters, arg_vasms)
                .set_type(return_type)
        },
        FunctionCall::Anonymous(details) => {
            context.vasm()
                .call_function_anonymous(check_location, &details.signature, details.function_offset, arg_vasms, context)
                .set_type(return_type)
        },
    };

    Some(result)
}

fn infer_function_parameters(function_parameters: &[Type], remaining_type_indexes_to_infer: &mut HashSet<usize>, actual_type: &Type, expected_type: &Type) -> Vec<Type> {
    let mut result = vec![];

    for (i, function_parameter) in function_parameters.iter().enumerate() {
        if let TypeContent::FunctionParameter(info) = function_parameter.content() {
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