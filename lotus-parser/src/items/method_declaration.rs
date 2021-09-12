use parsable::parsable;
use crate::{items::Visibility, program::{FunctionBlueprint, MethodDetails, ProgramContext, RESULT_VAR_NAME, ScopeKind, StructInfo, THIS_VAR_NAME, VariableKind, display_join, insert_in_vec_hashmap}, utils::Link};
use super::{EventCallbackQualifier, FunctionCondition, FunctionContent, FunctionDeclaration, FunctionSignature, Identifier, Statement, StatementList, TypeDeclaration, TypeQualifier, VarPath, VarRefPrefix};

#[parsable]
pub struct MethodDeclaration {
    pub content: FunctionContent
}

impl MethodDeclaration {
    pub fn process_signature(&self, context: &mut ProgramContext) {
        let type_id = context.current_type.unwrap();
        let mut function_blueprint = self.content.process_signature(context);
        let mut type_blueprint = context.current_type.unwrap();
        let is_static = function_blueprint.borrow().is_static();
        let name = function_blueprint.borrow().name.clone();

        function_blueprint.borrow_mut().visibility = Visibility::Member;

        let method_details = MethodDetails {
            name: self.content.name.clone(),
            owner: type_blueprint.clone(),
            content: function_blueprint,
        };

        let mut index_map = match is_static {
            true => &mut type_blueprint.borrow_mut().static_methods,
            false => &mut type_blueprint.borrow_mut().methods
        };

        if index_map.insert(name.to_string(), method_details).is_some() {
            let s = match is_static {
                true => "static ",
                false => ""
            };
            context.errors.add(self, format!("duplicate {}method `{}`", s, &name));
        }
    }

    pub fn process_body(&self, context: &mut ProgramContext) {
        self.content.process_body(context);
    }
}