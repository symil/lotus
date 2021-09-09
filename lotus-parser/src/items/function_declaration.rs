use parsable::parsable;
use crate::{generation::{Wat}, items::Visibility, program::{ProgramContext, ScopeKind, TypeOld, VariableKind, Wasm, RESULT_VAR_NAME}};
use super::{FullType, FunctionContent, FunctionSignature, Identifier, Statement, StatementList, VisibilityWrapper};

#[parsable]
pub struct FunctionDeclaration {
    pub visibility: VisibilityWrapper,
    #[parsable(prefix="fn")]
    pub content: FunctionContent
}

impl FunctionDeclaration {
    pub fn process_signature(&self, context: &mut ProgramContext) {
        let mut function_blueprint = self.content.process_signature(context);

        function_blueprint.visibility = self.visibility.value.unwrap_or(Visibility::Private);

        if function_blueprint.name.as_str() == "main" {
            if !function_blueprint.arguments.is_empty() {
                context.errors.add(self, format!("main function must not take any argument"));
            }

            if function_blueprint.return_type.is_some() {
                context.errors.add(self, format!("main function must not have a return type"));
            }

            if function_blueprint.visibility != Visibility::Export {
                context.errors.add(self, format!("main function must be declared with the `export` visibility"));
            }
        }

        if context.functions.get_by_name(&function_blueprint.name).is_some() {
            context.errors.add(self, format!("duplicate function declaration `{}`", &function_blueprint.name));
        }
        
        context.functions.insert(function_blueprint);
    }

    pub fn process_body(&self, context: &mut ProgramContext) {
        self.content.process_body(context);
    }
}