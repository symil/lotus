use parsable::parsable;
use crate::program::{ProgramContext, Vasm};
use super::{ParsedTemplateStringExpressionFragment, ParsedTemplateStringLiteralFragment};

#[parsable]
pub enum ParsedTemplateStringFragment {
    String(ParsedTemplateStringLiteralFragment),
    #[parsable(consume_spaces=false)]
    Expression(ParsedTemplateStringExpressionFragment)
}

impl ParsedTemplateStringFragment {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Vasm> {
        match self {
            ParsedTemplateStringFragment::String(string_fragment) => string_fragment.process(context),
            ParsedTemplateStringFragment::Expression(expression_fragment) => expression_fragment.process(context),
        }
    }
}