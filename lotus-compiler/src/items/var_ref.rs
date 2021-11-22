use parsable::parsable;
use colored::*;
use crate::{items::{process_field_access, process_function_call, process_method_call}, program::{AccessType, AnonymousFunctionCallDetails, BuiltinInterface, FieldKind, FunctionCall, NamedFunctionCallDetails, ProgramContext, THIS_VAR_NAME, Type, VI, VariableKind, Vasm}, vasm};
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
                                Some(function_vasm) => Some(Vasm::merge(vec![vasm![VI::get_var(&var_info)], function_vasm])),
                                None => None,
                            }
                        },
                        _ => context.errors.add_and_none(&self.name, format!("expected function, got `{}`", var_info.ty()))
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
                        None => context.errors.add_and_none(&self.name, format!("undefined function `{}`", &self.name)),
                    },
                },
                None => match context.access_var(&self.name) {
                    Some(var_info) => match access_type {
                        AccessType::Get => Some(Vasm::new(var_info.ty().clone(), vec![], vec![VI::get_var(&var_info)])),
                        AccessType::Set(_) => Some(Vasm::new(var_info.ty().clone(), vec![], vec![VI::set_var_from_stack(&var_info)])),
                    },
                    None => match context.functions.get_by_identifier(&self.name) {
                        Some(function_wrapped) => function_wrapped.with_ref(|function_unwrapped| {
                            match function_unwrapped.parameters.is_empty() {
                                true => Some(Vasm::new(Type::Function(Box::new(function_unwrapped.signature.clone())), vec![], vec![VI::function_index(&function_wrapped, &[])])),
                                false => context.errors.add_and_none(&self.name, format!("cannot use functions with parameters as variables for now")),
                            }
                        }),
                        None => match self.name.as_str() {
                            THIS_VAR_NAME => context.errors.add_and_none(&self.name, format!("no {} value can be referenced in this context", THIS_VAR_NAME.bold())),
                            _ => context.errors.add_and_none(&self.name, format!("undefined variable `{}`", self.name.as_str().bold()))
                        },
                    }
                },
            },
        }
    }
}