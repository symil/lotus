use parsable::parsable;
use crate::program::{ProgramContext, Wasm};
use super::{BinaryOperation, FullType};

#[parsable]
pub struct Expression {
    pub operation: BinaryOperation,
    #[parsable(prefix="as")]
    pub as_type: Option<FullType>
}

impl Expression {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Wasm> {
        self.operation.process(context)
    }
}