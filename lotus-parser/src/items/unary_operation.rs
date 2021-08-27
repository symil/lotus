use parsable::parsable;
use crate::{generation::{NULL_ADDR, Wat}, program::{ProgramContext, Type, Wasm}, wat};
use super::{Operand, UnaryOperator};

#[parsable]
pub struct UnaryOperation {
    pub operator: UnaryOperator,
    pub operand: Operand
}

impl UnaryOperation {
    pub fn has_side_effects(&self) -> bool {
        self.operand.has_side_effects()
    }

    pub fn process(&self, context: &mut ProgramContext) -> Option<Wasm> {
        let mut result = None;

        if let Some(operand_wasm) = self.operand.process(context) {
            if let Some(operator_wasm) = self.operator.process(&operand_wasm.ty, context) {
                result = Some(Wasm::merge(operator_wasm.ty.clone(), vec![operand_wasm, operator_wasm]));
            }
        }

        result
    }
}