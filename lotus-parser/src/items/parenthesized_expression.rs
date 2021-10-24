use parsable::parsable;
use crate::program::{ProgramContext, Type, Vasm};

use super::{Expression, Identifier};

#[parsable]
pub struct ParenthesizedExpression {
    #[parsable(brackets="()")]
    pub expression: Box<Expression>
}

impl ParenthesizedExpression {
    pub fn has_side_effects(&self) -> bool {
        self.expression.has_side_effects()
    }

    pub fn collected_instancied_type_names(&self, list: &mut Vec<Identifier>) {
        self.expression.collected_instancied_type_names(list);
    }

    pub fn process(&self, type_hint: Option<&Type>, context: &mut ProgramContext) -> Option<Vasm> {
        self.expression.process(type_hint, context)
    }
}