use parsable::parsable;
use crate::{generation::Wat, program::{ProgramContext, Type, Wasm}};
use super::{BinaryOperator, Operand, FullType};

#[parsable]
pub struct Operation {
    pub first: Operand,
    pub others: Vec<(BinaryOperator, Operand)>,
    #[parsable(prefix="as")]
    pub as_type: Option<FullType>
}

impl Operation {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Wasm> {
        todo!()
    }
}