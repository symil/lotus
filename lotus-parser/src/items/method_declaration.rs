use parsable::parsable;
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

            if index_map.insert(name.to_string(), func_ref).is_some() {
                let s = match is_static {
                    true => "static ",
                    false => ""
                };
                context.errors.add(self, format!("duplicate {}method `{}`", s, &name));
            }
        });
    }

    pub fn process_body(&self, context: &mut ProgramContext) {
        self.content.process_body(context);
    }
}