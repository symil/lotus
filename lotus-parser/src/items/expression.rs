use parsable::parsable;
use crate::program::{ProgramContext, TypeOld, IrFragment};
use super::{BinaryOperation, FullType};

#[parsable]
pub struct Expression {
    pub operation: BinaryOperation,
    #[parsable(prefix="as")]
    pub as_type: Option<FullType>
}

impl Expression {
    pub fn has_side_effects(&self) -> bool {
        self.operation.has_side_effects()
    }

    pub fn process(&self, context: &mut ProgramContext) -> Option<IrFragment> {
        let mut result = None;

        if let Some(wasm) = self.operation.process(context) {
            result = match &self.as_type {
                Some(as_type) => match as_type.process(context) {
                    Some(new_type) => Some(IrFragment::merge(new_type, vec![wasm])),
                    None => None
                },
                None => Some(wasm),
            }
        }

        result
    }
}