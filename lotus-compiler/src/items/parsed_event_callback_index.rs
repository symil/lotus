use parsable::{parsable, ItemLocation};
use crate::program::{Vasm, ProgramContext};
use super::{ParsedOpeningSquareBracket, ParsedExpression, ParsedClosingSquareBracket, unwrap_item};

#[parsable]
pub struct ParsedEventCallbackIndex {
    pub opening_bracket: ParsedOpeningSquareBracket,
    pub expression: Option<ParsedExpression>,
    pub closing_bracket: Option<ParsedClosingSquareBracket>
}

impl ParsedEventCallbackIndex {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Vasm> {
        let expression = unwrap_item(&self.expression, &self.opening_bracket, context)?;
        let closing_bracket = unwrap_item(&self.closing_bracket, self, context)?;
        let int_type = context.int_type();
        let mut result = None;

        if let Some(vasm) = expression.process(Some(&int_type), context) {
            if !vasm.ty.is_int() {
                context.errors.type_mismatch(expression, &int_type, &vasm.ty);
            } else {
                result = Some(vasm);
            }
        }

        result
    }
}