use parsable::parsable;
use crate::program::{ProgramContext, Vasm};
use super::make_string_value_from_literal;

#[parsable]
pub struct ParsedTemplateStringLiteralFragment {
    #[parsable(regex=r"[^`$]+")]
    pub content: String
}

impl ParsedTemplateStringLiteralFragment {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Vasm> {
        make_string_value_from_literal(self, &self.content, context)
    }
}