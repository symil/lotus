use parsable::{DataLocation, parsable};
use crate::program::{ProgramContext, Type, Vasm};
use super::{BlockExpression, Expression};

#[parsable]
pub enum FunctionLiteralBody {
    Block(BlockExpression),
    Expression(Expression)
}

impl FunctionLiteralBody {
    pub fn process(&self, type_hint: Option<&Type>, context: &mut ProgramContext) -> Option<Vasm> {
        match self {
            FunctionLiteralBody::Block(block) => block.process(type_hint, context),
            FunctionLiteralBody::Expression(expr) => expr.process(type_hint, context),
        }
    }
}