use parsable::parsable;
use crate::{generation::{Wat}, items::Visibility, program::{FunctionBlueprint, MethodDetails, ProgramContext, RESULT_VAR_NAME, ScopeKind, StructInfo, THIS_VAR_NAME, VariableKind, display_join, get_builtin_method_info, insert_in_vec_hashmap}};
use super::{EventCallbackQualifier, FunctionCondition, FunctionContent, FunctionDeclaration, FunctionSignature, Identifier, Statement, StatementList, TypeDeclaration, TypeQualifier, VarPath, VarRefPrefix};

#[parsable]
pub struct MethodDeclaration {
    pub content: FunctionContent
}

impl MethodDeclaration {
    pub fn process_signature(&self, context: &mut ProgramContext) {
        let type_id = context.current_type.unwrap();
        let mut method_blueprint = self.content.process_signature(context);
        let mut type_blueprint = context.types.get_mut_by_id(type_id);
        let mut index_map = match method_blueprint.is_static() {
            true => &mut type_blueprint.static_methods,
            false => &mut type_blueprint.methods
        };
        let method_details = MethodDetails {
            name: self.content.name.clone(),
            function_id: method_blueprint.function_id,
            owner_type_id: type_id,
        };

        method_blueprint.visibility = Visibility::Member;

        if index_map.insert(method_blueprint.name.to_string(), method_details).is_some() {
            let s = match method_blueprint.is_static() {
                true => "static ",
                false => ""
            };
            context.errors.add(self, format!("duplicate {}method `{}`", s, &method_blueprint.name));
        }

        context.functions.insert(method_blueprint);
    }

    pub fn process_body(&self, context: &mut ProgramContext) {
        self.content.process_body(context);
    }
}