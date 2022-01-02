use parsable::parsable;
use crate::{program::{ProgramContext, Type, Vasm}, wat};
use super::{Identifier, Operand, UnaryOperatorWrapper};

#[parsable]
pub struct UnaryOperation {
    pub operator: UnaryOperatorWrapper,
    pub operand: Box<Operand>
}

impl UnaryOperation {
    pub fn collected_instancied_type_names(&self, list: &mut Vec<Identifier>, context: &mut ProgramContext) {
        self.operand.collected_instancied_type_names(list, context);
    }

    pub fn process(&self, type_hint: Option<&Type>, context: &mut ProgramContext) -> Option<Vasm> {
        let mut result = None;

        if let Some(operand_vasm) = self.operand.process(type_hint, context) {
            if let Some(operator_vasm) = self.operator.process(&operand_vasm.ty, context) {
                result = Some(context.vasm()
                    .append(operand_vasm)
                    .append(operator_vasm)
                );
            }
        }

        result
    }
}