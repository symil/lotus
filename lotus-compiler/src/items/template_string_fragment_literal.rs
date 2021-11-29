use parsable::parsable;
use crate::program::{ProgramContext, Vasm};
use super::make_string_value_from_literal;

#[parsable]
pub struct TemplateStringFragmentLiteral {
    #[parsable(regex=r"[^`$]+")]
    pub content: String
}

impl TemplateStringFragmentLiteral {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Vasm> {
        make_string_value_from_literal(Some(self), &self.content, context)
    }
}