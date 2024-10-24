use colored::Colorize;
use parsable::parsable;
use crate::{program::{BuiltinInterface, BuiltinType, IS_NONE_METHOD_NAME, ProgramContext, Type, Vasm}, wat};

#[parsable]
pub struct ParsedUnaryOperator {
    pub token: ParsedUnaryOperatorToken
}

#[parsable(impl_display=true)]
pub enum ParsedUnaryOperatorToken {
    BooleanNot = "!",
    BinaryNot = "~"
}

impl ParsedUnaryOperator {
    pub fn process(&self, operand_type: &Type, context: &mut ProgramContext) -> Option<Vasm> {
        match &self.token {
            ParsedUnaryOperatorToken::BooleanNot => {
                if operand_type.is_void() {
                    context.errors.generic(self, format!("cannot apply `{}` operator to untyped expression", "!".bold()));
                    None
                } else if !operand_type.is_undefined() {
                    Some(context.vasm()
                        .call_regular_method(operand_type, IS_NONE_METHOD_NAME, &[], vec![], context)
                        .set_type(context.bool_type())
                    )
                } else {
                    None
                }

            },
            ParsedUnaryOperatorToken::BinaryNot => operand_type.call_builtin_interface_no_arg(self, BuiltinInterface::Not, context),
        }
    }
}