use parsable::parsable;
use crate::{generation::{NULL_ADDR, ToWat, ToWatVec, Wat}, program::{ARRAY_GET_LENGTH_FUNC_NAME, BuiltinInterface, ProgramContext, Type, TypeOld, IrFragment}, wat};

#[parsable]
pub struct UnaryOperatorWrapper {
    pub value: UnaryOperator
}

#[parsable(impl_display=true)]
pub enum UnaryOperator {
    Not = "!",
    ToBool = "?",
    Plus = "+",
    Minus = "-"
}

impl UnaryOperatorWrapper {
    pub fn process(&self, operand_type: &Type, context: &mut ProgramContext) -> Option<IrFragment> {
        let required_interface = match &self.value {
            UnaryOperator::Not => BuiltinInterface::Not,
            UnaryOperator::ToBool => BuiltinInterface::ToBool,
            UnaryOperator::Plus => BuiltinInterface::Plus,
            UnaryOperator::Minus => BuiltinInterface::Minus,
        };

        context.call_builtin_interface_no_arg(self, required_interface, operand_type)
    }
}