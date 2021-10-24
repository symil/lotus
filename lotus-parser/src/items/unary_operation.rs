use parsable::parsable;
use crate::{program::{ProgramContext, Type, Vasm}, wat};
use super::{Identifier, Operand, UnaryOperatorWrapper};

#[parsable]
pub struct UnaryOperation {
    pub operator: UnaryOperatorWrapper,
    pub operand: Operand
}

impl UnaryOperation {
    pub fn has_side_effects(&self) -> bool {
        self.operand.has_side_effects()
    }

    pub fn collected_instancied_type_names(&self, list: &mut Vec<Identifier>) {
        self.operand.collected_instancied_type_names(list);
    }

    pub fn process(&self, type_hint: Option<&Type>, context: &mut ProgramContext) -> Option<Vasm> {
        let mut result = None;

        if let Some(operand_wasm) = self.operand.process(type_hint, context) {
            if let Some(operator_wasm) = self.operator.process(&operand_wasm.ty, context) {
                result = Some(Vasm::merge(vec![operand_wasm, operator_wasm]));
            }
        }

        result
    }
}