use std::collections::HashMap;

use parsable::parsable;
use colored::*;
use crate::{items::Visibility, program::{DEFAULT_METHOD_NAME, FuncRef, FunctionBlueprint, ProgramContext, ScopeKind, THIS_VAR_NAME, VariableKind, display_join, hashmap_get_or_insert_with, insert_in_vec_hashmap}, utils::Link};
use super::{EventCallbackQualifier, FunctionCondition, FunctionContent, FunctionDeclaration, FunctionSignature, Identifier, BlockExpression, TypeDeclaration, TypeQualifier, VarPath};

#[parsable]
pub struct MethodDeclaration {
    pub content: FunctionContent
}

impl MethodDeclaration {
    pub fn is_autogen(&self) -> bool {
        self.content.is_autogen()
    }

    pub fn process_signature(&self, context: &mut ProgramContext) {
        let function_wrapped = self.content.process_signature(context);
        let mut type_wrapped = context.get_current_type().unwrap();

        function_wrapped.with_mut(|mut function_unwrapped| {
            function_unwrapped.visibility = Visibility::None;

            let is_static = function_unwrapped.is_static();
            let is_dynamic = function_unwrapped.is_dynamic();
            let name = function_unwrapped.name.clone();
            let mut method_details = function_unwrapped.method_details.as_mut().unwrap();
            let prev_opt = type_wrapped.with_mut(|mut type_unwrapped| {
                if let Some((qualifier, event_type_wrapped)) = &method_details.event_callback_details {
                    let event_map = hashmap_get_or_insert_with(&mut type_unwrapped.event_callbacks, event_type_wrapped, || HashMap::new());
                    let callback_list = hashmap_get_or_insert_with(event_map, qualifier, || vec![]);

                    callback_list.push(function_wrapped.clone());

                    None
                } else {
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
                    } else if let Some(autogen_type_blueprint) = &context.autogen_type {
                        method_details.first_declared_by = Some(autogen_type_blueprint.clone());
                    }

                    let should_insert = index_map.get(name.as_str()).is_none() || !self.is_autogen();

                    match should_insert {
                        true => index_map.insert(name.to_string(), func_ref),
                        false => None,
                    }
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

                        if prev_method_details.owner_type.as_ref().unwrap() == &type_wrapped {
                            // The type declares the same method twice
                            context.errors.add_generic(self, format!("duplicate {}method `{}`", s, name.as_str().bold()));
                        } else {
                            let parent_class_name = prev_method_details.owner_type.as_ref().unwrap().borrow().name.to_string();

                            if !is_dynamic && is_prev_dynamic {
                                context.errors.add_generic(self, format!("method `{}` is dynamic, but was declared as not dynamic by parent type `{}`", name.as_str().bold(), parent_class_name.bold()));
                            } else if is_dynamic && !is_prev_dynamic {
                                context.errors.add_generic(self, format!("method `{}` is not dynamic, but was declared as dynamic by parent type `{}`", name.as_str().bold(), parent_class_name.bold()));
                            } else if is_dynamic && is_prev_dynamic {
                                let prev_signature = &prev_unwrapped.signature;
                                let current_signature = &function_unwrapped.signature;

                                if current_signature != prev_signature {
                                    context.errors.add_generic(self, format!("dynamic method signature mismatch: expected `{}`, got `{}`", prev_signature, current_signature));
                                }
                            } else {
                                context.errors.add_generic(self, format!("duplicate {}method `{}` (already declared by parent type `{}`)", s, name.as_str().bold(), parent_class_name.bold()));
                            }
                        }
                    });
                }
            });
        });
    }

    pub fn process_body(&self, context: &mut ProgramContext) {
        self.content.process_body(context);
    }
}