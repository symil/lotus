use parsable::parsable;
use crate::{program::{Type, Vasm, ProgramContext, SELF_VAR_NAME}, language_server::FieldCompletionOptions};
use super::{ParsedExpression, ParsedType, Identifier, ParsedColonToken, ParsedEqualToken, ParsedCommaToken, unwrap_item, ParsedVarTypeDeclaration, ParsedDefaultValueAssignment};

#[parsable]
pub struct ParsedFieldDeclaration {
    pub name: Identifier,
    pub ty: Option<ParsedVarTypeDeclaration>,
    pub default_value: Option<ParsedDefaultValueAssignment>,
    pub comma: Option<ParsedCommaToken>,
}

impl ParsedFieldDeclaration {
    pub fn process(&self, parent_type: Option<&Type>, context: &mut ProgramContext) {
        if self.comma.is_none() && self.ty.is_none() {
            if let Some(ty) = parent_type {
                context.completion_provider.add_field_completion(&self.name, ty, Some(&FieldCompletionOptions {
                    show_methods: false,
                    insert_arguments: false,
                    hide_private: true,
                    prefix: "self.",
                    suffix: " = "
                }));
            }
        }
    }
}