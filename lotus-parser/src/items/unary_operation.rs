use parsable::parsable;
use crate::{generation::{ARRAY_LENGTH_FUNC_NAME, NULL_ADDR, ToWat, ToWatVec, Wat}, merge, program::{ProgramContext, Type, Wasm}, wat};
use super::{Operand, UnaryOperator};

#[parsable]
pub struct UnaryOperation {
    pub operator: UnaryOperator,
    pub operand: Operand
}

impl UnaryOperation {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Wasm> {
        let mut result = None;

        if let Some(operand_wasm) = self.operand.process(context) {
            if let Some(operator_wasm) = self.operator.process(&operand_wasm.ty, context) {
                result = Some(Wasm::typed(operator_wasm.ty, merge![operand_wasm.wat, operator_wasm.wat]));
            }
        }

        result
    }
}