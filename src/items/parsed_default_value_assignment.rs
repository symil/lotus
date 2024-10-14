use parsable::parsable;
use crate::program::{ProgramContext, Type, Vasm};
use super::{ParsedEqualToken, ParsedExpression, unwrap_item};

#[parsable]
pub struct ParsedDefaultValueAssignment {
    pub equal: ParsedEqualToken,
    pub expression: Option<ParsedExpression>,
}

impl ParsedDefaultValueAssignment {
    pub fn process(&self, type_hint: Option<&Type>, context: &mut ProgramContext) -> Option<Vasm> {
        let expression = unwrap_item(&self.expression, self, context)?;

        expression.process(type_hint, context)
    }
}