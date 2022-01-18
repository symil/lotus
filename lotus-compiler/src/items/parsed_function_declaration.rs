use parsable::{parsable, Token};
use crate::{items::ParsedVisibilityToken, program::{ProgramContext, ScopeKind, VariableKind, Visibility, FN_KEYWORD}};
use super::{ParsedType, ParsedFunctionOrMethodContent, ParsedFunctionSignature, Identifier, ParsedBlockExpression, ParsedVisibility};

#[parsable]
pub struct ParsedFunctionDeclaration {
    pub visibility: Option<ParsedVisibility>,
    pub fn_keyword: Token<FN_KEYWORD>,
    pub content: ParsedFunctionOrMethodContent
}

impl ParsedFunctionDeclaration {
    pub fn process_signature(&self, context: &mut ProgramContext) {
        if context.functions.get_by_identifier(&self.content.name).is_some() {
            context.errors.generic(self, format!("duplicate function declaration `{}`", &self.content.name));
        }

        let mut function_wrapped = self.content.process_signature(context);

        let name = function_wrapped.with_mut(|mut function_unwrapped| {
            function_unwrapped.visibility = ParsedVisibility::process_or(&self.visibility, Visibility::Private);

            if function_unwrapped.name.as_str() == "main" {
                if !function_unwrapped.signature.argument_types.is_empty() {
                    context.errors.generic(self, format!("main function must not take any argument"));
                }

                if !function_unwrapped.signature.return_type.is_void() {
                    context.errors.generic(self, format!("main function must not have a return type"));
                }

                if function_unwrapped.visibility != Visibility::Export {
                    context.errors.generic(self, format!("main function must be declared with the `export` visibility"));
                }
            }

            function_unwrapped.name.clone()
        });
    }

    pub fn process_body(&self, context: &mut ProgramContext) {
        self.content.process_body(context);
    }
}