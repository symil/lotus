use colored::Colorize;
use parsable::parsable;
use crate::{program::{BuiltinInterface, BuiltinType, IS_NONE_FUNC_NAME, ProgramContext, Type, VI, Vasm}, wat};

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
                if operand_type.is_bool() {
                    Some(Vasm::new(context.bool_type(), vec![], vec![VI::raw(wat!["i32.eqz"])]))
                } else if let Some(option_type) = operand_type.get_builtin_type_parameter(BuiltinType::Option) {
                    Some(Vasm::new(context.bool_type(), vec![], vec![VI::call_regular_method(option_type, IS_NONE_FUNC_NAME, &[], vec![], context)]))
                } else {
                    context.errors.add(self, format!("expected `{}` or `{}`, got `{}`", BuiltinType::Bool.get_name(), "Option<_>".bold(), operand_type));
                    None
                }

            },
            UnaryOperator::BinaryNot => operand_type.call_builtin_interface_no_arg(self, BuiltinInterface::Not, context),
        }
    }
}