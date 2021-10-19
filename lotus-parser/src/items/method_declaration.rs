use parsable::parsable;
use colored::*;
use crate::{items::Visibility, program::{FuncRef, FunctionBlueprint, ProgramContext, RESULT_VAR_NAME, ScopeKind, THIS_VAR_NAME, VariableKind, display_join, insert_in_vec_hashmap}, utils::Link};
use super::{EventCallbackQualifier, FunctionCondition, FunctionContent, FunctionDeclaration, FunctionSignature, Identifier, Statement, StatementList, TypeDeclaration, TypeQualifier, VarPath};

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

        function_wrapped.borrow_mut().visibility = Visibility::Member;

        function_wrapped.with_ref(|function_unwrapped| {
            let is_static = function_unwrapped.is_static();
            let is_dynamic = function_unwrapped.is_dynamic;
            let name = function_unwrapped.name.clone();

            let prev_opt = type_wrapped.with_mut(|mut type_unwrapped| {
                let func_ref = FuncRef {
                    function: function_wrapped.clone(),
                    this_type: type_unwrapped.self_type.clone(),
                };

                let mut index_map = match is_static {
                    true => &mut type_unwrapped.static_methods,
                    false => &mut type_unwrapped.regular_methods
                };

                index_map.insert(name.to_string(), func_ref)
            });

            type_wrapped.with_ref(|type_unwrapped| {
                if let Some(prev) = prev_opt {
                    prev.function.with_ref(|prev_unwrapped| {
                        let parent_class_name = prev_unwrapped.owner_type.as_ref().unwrap().borrow().name.to_string();
                        let is_prev_dynamic = prev_unwrapped.is_dynamic;

                        if !is_dynamic && is_prev_dynamic {
                            context.errors.add(self, format!("method `{}` is dynamic, but was declared as not dynamic by parent type `{}`", name.as_str().bold(), parent_class_name.bold()));
                        } else if is_dynamic && !is_prev_dynamic {
                            context.errors.add(self, format!("method `{}` is not dynamic, but was declared as dynamic by parent type `{}`", name.as_str().bold(), parent_class_name.bold()));
                        } else if is_dynamic && is_prev_dynamic {
                            let prev_signature = prev_unwrapped.get_signature();
                            let current_signature = function_unwrapped.get_signature();

                            if current_signature != prev_signature {
                                context.errors.add(self, format!("dynamic method signature mismatch: expected `{}`, got `{}`", prev_signature, current_signature));
                            }
                        } else {
                            let s = match is_static {
                                true => "static ",
                                false => ""
                            };
                            context.errors.add(self, format!("duplicate {}method `{}` (already declared by parent type `{}`)", s, name.as_str().bold(), parent_class_name.bold()));
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