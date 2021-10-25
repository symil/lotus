use parsable::parsable;
use crate::{program::{BuiltinInterface, ProgramContext, Type, VI, Vasm}, wat};

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
                match operand_type.call_builtin_interface_no_arg(self, BuiltinInterface::ToBool, context) {
                    Some(mut vasm) => {
                        vasm.extend(Vasm::new(context.bool_type(), vec![], vec![VI::raw(wat!["i32.eqz"])]));
                        Some(vasm)
                    },
                    None => None,
                }
            },
            UnaryOperator::BinaryNot => operand_type.call_builtin_interface_no_arg(self, BuiltinInterface::Not, context),
        }
    }
}