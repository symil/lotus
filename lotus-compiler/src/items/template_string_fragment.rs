use parsable::parsable;
use crate::program::{ProgramContext, Vasm};
use super::{TemplateStringFragmentExpression, TemplateStringFragmentLiteral};

#[parsable]
pub enum TemplateStringFragment {
    String(TemplateStringFragmentLiteral),
    #[parsable(consume_spaces=false)]
    Expression(TemplateStringFragmentExpression)
}

impl TemplateStringFragment {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Vasm> {
        match self {
            TemplateStringFragment::String(string_fragment) => string_fragment.process(context),
            TemplateStringFragment::Expression(expression_fragment) => expression_fragment.process(context),
        }
    }
}