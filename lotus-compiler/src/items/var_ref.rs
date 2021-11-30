use parsable::{DataLocation, parsable};
use colored::*;
use crate::{items::{ObjectLiteral, ParsedTypeSingle, ParsedTypeWithoutSuffix, ParsedValueType, TypeArguments, process_field_access, process_function_call, process_method_call, type_arguments}, program::{AccessType, AnonymousFunctionCallDetails, BuiltinInterface, FieldKind, FunctionCall, NamedFunctionCallDetails, ProgramContext, SELF_VAR_NAME, Type, VI, VariableKind, Vasm}, vasm};
use super::{ArgumentList, FieldOrMethodAccess, ParsedType, Identifier, VarPrefix, VarPrefixWrapper};

#[parsable]
pub struct VarRef {
    pub prefix: Option<VarPrefixWrapper>,
    pub name: Identifier,
    pub args: Option<ArgumentList>
}

impl VarRef {
    pub fn has_side_effects(&self) -> bool {
        self.args.is_some()
    }

    pub fn as_single_local_variable(&self) -> Option<&Identifier> {
        match &self.prefix {
            Some(_) => None,
            None => match &self.args {
                Some(_) => None,
                None => Some(&self.name),
            },
        }
    }

    pub fn collected_instancied_type_names(&self, list: &mut Vec<Identifier>) {
        
    }

    pub fn process(&self, type_hint: Option<&Type>, access_type: AccessType, context: &mut ProgramContext) -> Option<Vasm> {
        let current_function_level = Some(context.get_function_level());

        match &self.prefix {
            Some(prefix) => match prefix.process(context) {
                Some(prefix_vasm) => match &self.args {
                    Some(args) => match process_method_call(&prefix_vasm.ty, FieldKind::Regular, &self.name, &[], args, type_hint, access_type, context) {
                        Some(method_vasm) => Some(Vasm::merge(vec![prefix_vasm, method_vasm])),
                        None => None,
                    },
                    None => match process_field_access(&prefix_vasm.ty, FieldKind::Regular, &self.name, access_type, context) {
                        Some(field_vasm) => Some(Vasm::merge(vec![prefix_vasm, field_vasm])),
                        None => None,
                    },
                },
                None => None,
            },
            None => match &self.args {
                Some(args) => match context.access_var(&self.name) {
                    Some(var_info) => match &var_info.ty().clone() {
                        Type::Function(signature) => {
                            let function_call = FunctionCall::Anonymous(AnonymousFunctionCallDetails {
                                signature: Box::as_ref(signature).clone(),
                                function_offset: 0,
                            });

                            match process_function_call(&self.name, function_call, args, type_hint, access_type, context) {
                                Some(function_vasm) => Some(Vasm::merge(vec![vasm![VI::get_var(&var_info, current_function_level)], function_vasm])),
                                None => None,
                            }
                        },
                        _ => {
                            if !var_info.ty().is_undefined() {
                                context.errors.add_generic(&self.name, format!("expected function, got `{}`", var_info.ty()));
                            }

                            None
                        }
                    },
                    None => match context.functions.get_by_identifier(&self.name) {
                        Some(function_blueprint) => {
                            let function_call = FunctionCall::Named(NamedFunctionCallDetails {
                                caller_type: None,
                                function: function_blueprint.clone(),
                                parameters: vec![],
                            });

                            process_function_call(&self.name, function_call, args, type_hint, access_type, context)
                        },
                        None => context.errors.add_generic_and_none(&self.name, format!("undefined function `{}`", &self.name)),
                    },
                },
                None => match context.access_var(&self.name) {
                    Some(var_info) => match access_type {
                        AccessType::Get => Some(Vasm::new(var_info.ty().clone(), vec![], vec![VI::get_var(&var_info, current_function_level)])),
                        AccessType::Set(location) => Some(Vasm::new(var_info.ty().clone(), vec![], vec![VI::set_var(&var_info, current_function_level, vasm![VI::placeholder(location)])])),
                    },
                    None => match context.functions.get_by_identifier(&self.name) {
                        Some(function_wrapped) => function_wrapped.with_ref(|function_unwrapped| {
                            match function_unwrapped.parameters.is_empty() {
                                true => Some(Vasm::new(Type::Function(Box::new(function_unwrapped.signature.clone())), vec![], vec![VI::function_index(&function_wrapped, &[])])),
                                false => context.errors.add_generic_and_none(&self.name, format!("cannot use functions with parameters as variables for now")),
                            }
                        }),
                        None => match context.types.get_by_identifier(&self.name) {
                            Some(type_wrapped) => match type_wrapped.borrow().parameters.is_empty() && type_wrapped.borrow().is_class() {
                                true => {
                                    let type_arguments = TypeArguments::default();
                                    let mut parsed_type_value = ParsedValueType::default();
                                    parsed_type_value.name = self.name.clone();
                                    parsed_type_value.location = self.name.location.clone();
                                    let mut parsed_type = ParsedType::default();
                                    parsed_type.parsed_type = ParsedTypeWithoutSuffix::Single(ParsedTypeSingle::Value(parsed_type_value));
                                    let mut object_literal = ObjectLiteral::default();
                                    object_literal.object_type = parsed_type;

                                    object_literal.process(context)
                                },
                                false => context.errors.add_generic_and_none(&self.name, format!("undefined variable `{}`", self.name.as_str().bold())),
                            },
                            None => match self.name.as_str() {
                                SELF_VAR_NAME => context.errors.add_generic_and_none(&self.name, format!("no `{}` value can be referenced in this context", SELF_VAR_NAME.bold())),
                                _ => context.errors.add_generic_and_none(&self.name, format!("undefined variable `{}`", self.name.as_str().bold()))
                            },
                        }
                    }
                },
            },
        }
    }
}