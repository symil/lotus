use parsable::parsable;
use crate::program::{ProgramContext, Type, Wasm};
use super::{BinaryOperation, FullType};

#[parsable]
pub struct Expression {
    pub operation: BinaryOperation,
    #[parsable(prefix="as")]
    pub as_type: Option<FullType>
}

impl Expression {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Wasm> {
        let mut result = None;

        if let Some(wasm) = self.operation.process(context) {
            result = match &self.as_type {
                Some(as_type) => match Type::from_parsed_type(&as_type, context) {
                    Some(new_type) => Some(Wasm::typed(new_type, wasm.wat)),
                    None => None
                },
                None => Some(wasm),
            }
        }

        result
    }
}