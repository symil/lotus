use parsable::parsable;
use crate::program::{ProgramContext, Vasm};
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

    pub fn process(&self, context: &mut ProgramContext) -> Option<Vasm> {
        let mut result = None;

        if let Some(vasm) = self.operation.process(context) {
            result = match &self.as_type {
                Some(as_type) => match as_type.process(context) {
                    Some(new_type) => Some(Vasm::merge(vec![vasm, Vasm::new(new_type, vec![], vec![])])),
                    None => None
                },
                None => Some(vasm),
            }
        }

        result
    }
}