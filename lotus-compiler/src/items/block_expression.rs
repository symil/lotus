use parsable::parsable;
use crate::program::{ProgramContext, Type, Vasm};
use super::Expression;

#[parsable]
pub struct BlockExpression {
    #[parsable(brackets="{}", separator=";")]
    pub list: Vec<Expression>
}

impl BlockExpression {
    pub fn process(&self, hint: Option<&Type>, context: &mut ProgramContext) -> Option<Vasm> {
        None
    }
}