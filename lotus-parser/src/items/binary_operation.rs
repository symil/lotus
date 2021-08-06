use parsable::parsable;
use crate::{generation::Wat, program::{ExpressionType, ProgramContext}};
use super::{BinaryOperator, Operand, Type};

#[parsable]
pub struct Operation {
    pub first: Operand,
    pub others: Vec<(BinaryOperator, Operand)>,
    #[parsable(prefix="as")]
    pub as_type: Option<Type>
}

impl Operation {
    pub fn process(&self, context: &mut ProgramContext) -> Option<(ExpressionType, Wat)> {
        todo!()
    }
}