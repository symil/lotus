use parsable::parsable;
use crate::program::{ProgramContext, Vasm, Type};
use super::{ParsedArrow, ParsedExpression};

#[parsable]
pub struct ParsedMatchBlockBody {
    pub arrow: ParsedArrow,
    pub expression: Option<ParsedExpression>
}

impl ParsedMatchBlockBody {
    pub fn process(&self, type_hint: Option<&Type>, context: &mut ProgramContext) -> Option<Vasm> {
        let expression = match &self.expression {
            Some(expression) => expression,
            None => {
                context.errors.expected_expression(self);
                return None;
            },
        };

        expression.process(type_hint, context)
    }
}