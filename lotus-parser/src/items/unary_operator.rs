use parsable::parsable;
use crate::{program::{BuiltinInterface, ProgramContext, Type, Vasm}, wat};

#[parsable]
pub struct UnaryOperatorWrapper {
    pub value: UnaryOperator
}

#[parsable(impl_display=true)]
pub enum UnaryOperator {
    Not = "!",
    // Plus = "+",
    // Minus = "-"
}

impl UnaryOperatorWrapper {
    pub fn process(&self, operand_type: &Type, context: &mut ProgramContext) -> Option<Vasm> {
        let required_interface = match &self.value {
            UnaryOperator::Not => BuiltinInterface::Not,
            // UnaryOperator::Plus => BuiltinInterface::Plus,
            // UnaryOperator::Minus => BuiltinInterface::Minus,
        };

        context.call_builtin_interface_no_arg(self, required_interface, operand_type)
    }
}