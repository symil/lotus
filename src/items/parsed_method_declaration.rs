use std::collections::HashMap;
use parsable::parsable;
use colored::*;
use crate::{items::ParsedVisibilityToken, program::{FuncRef, FunctionBlueprint, ProgramContext, ScopeKind, SELF_VAR_NAME, VariableKind, display_join, hashmap_get_or_insert_with, insert_in_vec_hashmap, Visibility, TO_STRING_METHOD_NAME}, utils::Link};
use super::{ParsedEventCallbackQualifierKeyword, ParsedFunctionOrMethodContent, ParsedFunctionDeclaration, ParsedFunctionSignature, Identifier, ParsedBlockExpression, ParsedTypeDeclaration, ParsedTypeQualifier, ParsedVarPath};

#[parsable]
pub struct ParsedMethodDeclaration {
    pub content: ParsedFunctionOrMethodContent
}

impl ParsedMethodDeclaration {
    pub fn is_autogen(&self) -> bool {
        self.content.is_autogen()
    }

    pub fn process_signature(&self, context: &mut ProgramContext) {
        let function_wrapped = self.content.process_signature(context);
        let mut type_wrapped = context.get_current_type().unwrap();

        function_wrapped.with_mut(|mut function_unwrapped| {
            let is_static = function_unwrapped.is_static();
            let is_dynamic = function_unwrapped.is_dynamic();
            let name = function_unwrapped.name.clone();
            let mut method_details = function_unwrapped.method_details.as_mut().unwrap();
            let prev_opt = type_wrapped.with_mut(|mut type_unwrapped| {
                let func_ref = FuncRef {
                    function: function_wrapped.clone(),
                    this_type: type_unwrapped.self_type.clone(),
                };

                let mut index_map = match is_static {
                    true => &mut type_unwrapped.static_methods,
                    false => &mut type_unwrapped.regular_methods
                };

                if let Some(prev) = index_map.get(name.as_str()) {
                    method_details.first_declared_by = prev.function.borrow().method_details.as_ref().unwrap().first_declared_by.clone();

                    if self.is_autogen() {
                        prev.function.with_mut(|mut prev_unwrapped| {
                            prev_unwrapped.method_details.as_mut().unwrap().first_declared_by = context.autogen_type.clone();
                        });
                    }
                } else if let Some(autogen_type_blueprint) = &context.autogen_type {
                    method_details.first_declared_by = Some(autogen_type_blueprint.clone());
                } else {
                    method_details.first_declared_by = Some(type_wrapped.clone());
                }

                let should_insert = match index_map.get(name.as_str()) {
                    Some(prev) => prev.function.borrow().method_details.as_ref().unwrap().is_autogen || !self.is_autogen(),
                    None => true,
                };

                match should_insert {
                    true => index_map.insert(name.to_string(), func_ref),
                    false => None,
                }
            });

            type_wrapped.with_ref(|type_unwrapped| {
                if let Some(prev) = prev_opt {
                    prev.function.with_ref(|prev_unwrapped| {
                        let s = match is_static {
                            true => "static ",
                            false => ""
                        };

                        let is_prev_dynamic = prev_unwrapped.is_dynamic();
                        let prev_method_details = prev_unwrapped.method_details.as_ref().unwrap();
                        let is_prev_autogen = prev_method_details.is_autogen;
                        let is_prev_system = prev_method_details.visibility.is_system();

                        if prev_unwrapped.owner_type.as_ref().unwrap() == &type_wrapped {
                            // The type declares the same method twice
                            if !is_prev_autogen {
                                context.errors.generic(self, format!("duplicate {}method `{}`", s, name.as_str().bold()));
                            }
                        } else {
                            let parent_class_name = prev_unwrapped.owner_type.as_ref().unwrap().borrow().name.to_string();

                            if is_prev_dynamic {
                                let prev_signature = &prev_unwrapped.signature;
                                let current_signature = &function_unwrapped.signature;

                                if current_signature != prev_signature {
                                    context.errors.generic(self, format!("dynamic method signature mismatch: expected `{}`, got `{}`", prev_signature, current_signature));
                                }

                                function_unwrapped.method_details.as_mut().unwrap().dynamic_index = Some(-1);
                            } else if is_dynamic {
                                context.errors.generic(self, format!("method `{}` is dynamic, but was declared as not dynamic by parent type `{}`", name.as_str().bold(), parent_class_name.bold()));
                            } else if !is_prev_autogen && !is_prev_system {
                                context.errors.generic(self, format!("duplicate {}method `{}` (already declared by parent type `{}`)", s, name.as_str().bold(), parent_class_name.bold()));
                            }
                        }
                    });
                }
            });
        });
    }

    pub fn process_default_arguments(&self, context: &mut ProgramContext) {
        self.content.process_default_arguments(context);
    }

    pub fn process_body(&self, context: &mut ProgramContext) {
        self.content.process_body(context);
    }
}