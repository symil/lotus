use parsable::parsable;
use crate::{items::Visibility, program::{ProgramContext, ScopeKind, VariableKind, RESULT_VAR_NAME}};
use super::{ParsedType, FunctionContent, FunctionSignature, Identifier, BlockExpression, VisibilityWrapper};

#[parsable]
pub struct FunctionDeclaration {
    pub visibility: VisibilityWrapper,
    #[parsable(prefix="fn")]
    pub content: FunctionContent
}

impl FunctionDeclaration {
    pub fn process_signature(&self, context: &mut ProgramContext) {
        if context.functions.get_by_identifier(&self.content.name).is_some() {
            context.errors.add(self, format!("duplicate function declaration `{}`", &self.content.name));
        }

        let mut function_wrapped = self.content.process_signature(context);

        let name = function_wrapped.with_mut(|mut function_unwrapped| {
            function_unwrapped.visibility = self.visibility.value.unwrap_or(Visibility::Private);

            if function_unwrapped.name.as_str() == "main" {
                if !function_unwrapped.signature.argument_types.is_empty() {
                    context.errors.add(self, format!("main function must not take any argument"));
                }

                if !function_unwrapped.signature.return_type.is_void() {
                    context.errors.add(self, format!("main function must not have a return type"));
                }

                if function_unwrapped.visibility != Visibility::Export {
                    context.errors.add(self, format!("main function must be declared with the `export` visibility"));
                }
            }

            function_unwrapped.name.clone()
        });
    }

    pub fn process_body(&self, context: &mut ProgramContext) {
        let function_name = &self.content.name;
        let type_id = context.current_type.as_ref().and_then(|t| Some(t.borrow().type_id));

        context.current_function = Some(context.functions.get_by_location(function_name, type_id).clone());

        self.content.process_body(context);

        context.current_function = None;
    }
}