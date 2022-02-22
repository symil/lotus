use parsable::parsable;
use crate::program::{Type, Vasm, ProgramContext, SELF_VAR_NAME};
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
                context.completion_provider.add_field_completion(&self.name, ty, false, false, "self.")
            }
            // context.completion_provider.add_keyword_completion(&self.name, &[SELF_VAR_NAME]);
        }
    }
}