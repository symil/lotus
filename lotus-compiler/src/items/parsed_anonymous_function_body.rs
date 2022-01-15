use parsable::{ItemLocation, parsable};
use crate::program::{ProgramContext, Type, Vasm};
use super::{ParsedBlockExpression, ParsedExpression};

#[parsable]
pub enum ParsedAnonymousFunctionBody {
    Block(ParsedBlockExpression),
    Expression(ParsedExpression)
}

impl ParsedAnonymousFunctionBody {
    pub fn process(&self, type_hint: Option<&Type>, context: &mut ProgramContext) -> Option<Vasm> {
        match self {
            ParsedAnonymousFunctionBody::Block(block) => block.process(type_hint, context),
            ParsedAnonymousFunctionBody::Expression(expr) => expr.process(type_hint, context),
        }
    }
}