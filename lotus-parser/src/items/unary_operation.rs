use parsable::parsable;
use crate::{generation::{ARRAY_LENGTH_FUNC_NAME, NULL_ADDR, Wat}, program::{ProgramContext, Type, Wasm}, wat};
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
                let mut wat = vec![];

                wat.extend(operand_wasm.wat);
                wat.extend(operator_wasm.wat);

                result = Some(Wasm::typed(operator_wasm.ty, wat));
            }
        }

        result
    }
}