use colored::Colorize;
use parsable::parsable;
use crate::{program::{BuiltinInterface, BuiltinType, IS_NONE_METHOD_NAME, ProgramContext, Type, VI, Vasm}, wat};

#[parsable]
pub struct UnaryOperatorWrapper {
    pub value: UnaryOperator
}

#[parsable(impl_display=true)]
pub enum UnaryOperator {
    BooleanNot = "!",
    BinaryNot = "~"
}

impl UnaryOperatorWrapper {
    pub fn process(&self, operand_type: &Type, context: &mut ProgramContext) -> Option<Vasm> {
        match &self.value {
            UnaryOperator::BooleanNot => {
                if operand_type.is_void() {
                    context.errors.generic(self, format!("cannot apply `{}` operator to untyped expression", "!".bold()));
                    None
                } else if !operand_type.is_undefined() {
                    Some(Vasm::new(context.bool_type(), vec![], vec![VI::call_regular_method(operand_type, IS_NONE_METHOD_NAME, &[], vec![], context)]))
                } else {
                    None
                }

            },
            UnaryOperator::BinaryNot => operand_type.call_builtin_interface_no_arg(self, BuiltinInterface::Not, context),
        }
    }
}