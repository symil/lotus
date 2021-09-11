use parsable::parsable;
use crate::program::{ProgramContext, IrFragment};

use super::Expression;

#[parsable]
pub struct ParenthesizedExpression {
    #[parsable(brackets="()")]
    pub expression: Box<Expression>
}

impl ParenthesizedExpression {
    pub fn has_side_effects(&self) -> bool {
        self.expression.has_side_effects()
    }

    pub fn process(&self, context: &mut ProgramContext) -> Option<IrFragment> {
        self.expression.process(context)
    }
}