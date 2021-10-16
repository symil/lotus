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
        let is_static = function_wrapped.borrow().is_static();
        let name = function_wrapped.borrow().name.clone();

        function_wrapped.borrow_mut().visibility = Visibility::Member;

        type_wrapped.with_mut(|mut type_unwrapped| {
            let func_ref = FuncRef {
                function: function_wrapped.clone(),
                this_type: type_unwrapped.self_type.clone(),
            };
            
            let mut index_map = match is_static {
                true => &mut type_unwrapped.static_methods,
                false => &mut type_unwrapped.regular_methods
            };

            if let Some(prev) = index_map.insert(name.to_string(), func_ref) {
                let parent_class_name = prev.function.borrow().owner_type.as_ref().unwrap().borrow().name.to_string();
                let s = match is_static {
                    true => "static ",
                    false => ""
                };
                // context.errors.add(self, format!("duplicate {}method `{}` (already declared by parent type `{}`)", s, name.as_str().bold(), parent_class_name.bold()));
            }
        });
    }

    pub fn process_body(&self, context: &mut ProgramContext) {
        self.content.process_body(context);
    }
}