use parsable::{DataLocation, parsable};
use colored::*;
use crate::{items::{ParsedObjectLiteral, ParsedTypeSingle, ParsedTypeWithoutSuffix, ParsedValueType, ParsedTypeArguments, process_field_access, process_function_call, process_method_call}, program::{AccessType, AnonymousFunctionCallDetails, BuiltinInterface, FieldKind, FunctionCall, NamedFunctionCallDetails, ProgramContext, SELF_VAR_NAME, Type, VariableKind, Vasm, TypeContent}};
use super::{ParsedArgumentList, ParsedFieldOrMethodAccess, ParsedType, Identifier, ParsedVarPrefixToken, ParsedVarPrefix, ParsedIdentifierWrapper};

#[parsable]
pub struct ParsedVarRef {
    pub name: ParsedIdentifierWrapper,
    pub arguments: Option<ParsedArgumentList>
}

impl ParsedVarRef {
    pub fn has_side_effects(&self) -> bool {
        self.arguments.is_some()
    }

    pub fn collected_instancied_type_names(&self, list: &mut Vec<String>) {
        
    }

    pub fn process(&self, type_hint: Option<&Type>, access_type: AccessType, context: &mut ProgramContext) -> Option<Vasm> {
        let current_function_level = Some(context.get_function_level());
        let var_name = self.name.process(context)?;

        context.add_variable_completion_area(&var_name.location, self.arguments.is_none(), &type_hint);

        match &self.arguments {
            Some(args) => match context.access_var(&var_name) {
                Some(var_info) => match &var_info.ty().clone().content() {
                    TypeContent::Function(signature) => {
                        let function_call = FunctionCall::Anonymous(AnonymousFunctionCallDetails {
                            signature: signature.clone(),
                            function_offset: 0,
                        });

                        match process_function_call(&var_name, function_call, args, type_hint, access_type, context) {
                            Some(function_vasm) => Some(context.vasm()
                                .get_var(&var_info, current_function_level)
                                .append(function_vasm)
                            ),
                            None => None,
                        }
                    },
                    _ => {
                        if !var_info.ty().is_undefined() {
                            context.errors.generic(&var_name, format!("expected function, got `{}`", var_info.ty()));
                        }

                        None
                    }
                },
                None => match context.functions.get_by_identifier(&var_name) {
                    Some(function_blueprint) => {
                        let function_call = FunctionCall::Named(NamedFunctionCallDetails {
                            caller_type: None,
                            function: function_blueprint.clone(),
                            parameters: vec![],
                        });

                        process_function_call(&var_name, function_call, args, type_hint, access_type, context)
                    },
                    None => {
                        context.errors.generic(&var_name, format!("undefined function `{}`", &var_name));
                        None
                    },
                },
            },
            None => match context.access_var(&var_name) {
                Some(var_info) => match access_type {
                    AccessType::Get => Some(context.vasm()
                        .get_var(&var_info, current_function_level)
                        .set_type(var_info.ty().clone())
                    ),
                    AccessType::Set(location) => Some(context.vasm()
                        .set_var(&var_info, current_function_level, context.vasm().placeholder(location))
                        .set_type(var_info.ty().clone())
                    ),
                },
                None => match context.functions.get_by_identifier(&var_name) {
                    Some(function_wrapped) => function_wrapped.with_ref(|function_unwrapped| {
                        match function_unwrapped.parameters.is_empty() {
                            true => Some(context.vasm()
                                .function_index(&function_wrapped, &[])
                                .set_type(Type::function(&function_unwrapped.signature))
                            ),
                            false => {
                                context.errors.generic(&var_name, format!("cannot use functions with parameters as variables for now"));
                                None
                            },
                        }
                    }),
                    None => match context.types.get_by_identifier(&var_name) {
                        Some(type_wrapped) => match type_wrapped.borrow().parameters.is_empty() && type_wrapped.borrow().is_class() {
                            true => {
                                let type_arguments = ParsedTypeArguments::default();
                                let mut parsed_type_value = ParsedValueType::default();
                                parsed_type_value.name = var_name.into_owned();
                                parsed_type_value.location = parsed_type_value.name.location.clone();
                                let mut parsed_type = ParsedType::default();
                                parsed_type.parsed_type = ParsedTypeWithoutSuffix::Single(ParsedTypeSingle::Value(parsed_type_value));
                                let mut object_literal = ParsedObjectLiteral::default();
                                object_literal.object_type = parsed_type;

                                object_literal.process(context)
                            },
                            false => {
                                context.errors.generic(&var_name, format!("undefined variable `{}`", var_name.as_str().bold()));
                                None
                            },
                        },
                        None => match var_name.as_str() {
                            SELF_VAR_NAME => {
                                context.errors.generic(&var_name, format!("no `{}` value can be referenced in this context", SELF_VAR_NAME.bold()));
                                None
                            },
                            _ => {
                                context.errors.generic(&var_name, format!("undefined variable `{}`", var_name.as_str().bold()));
                                None
                            }
                        },
                    }
                }
            },
        }
    }
}