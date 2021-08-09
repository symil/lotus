use parsable::parsable;
use crate::{program::{ProgramContext, Type, Wasm}, wat, generation::{ARRAY_LENGTH_FUNC_NAME, NULL_ADDR, ToWat, ToWatVec, Wat}};

#[parsable]
pub struct UnaryOperator {
    pub token: UnaryOperatorToken
}

#[parsable(impl_display=true)]
pub enum UnaryOperatorToken {
    Not = "!",
    AsBool = "?",
    Plus = "+",
    Minus = "-"
}

impl UnaryOperator {
    pub fn process(&self, operand_type: &Type, context: &mut ProgramContext) -> Option<Wasm> {
        let wat_result = match &self.token {
            UnaryOperatorToken::AsBool => operand_type.to_bool(),
            UnaryOperatorToken::Not => match operand_type {
                Type::Pointer => Some(wat!["i32.eqz"]),
                Type::Boolean => Some(wat!["i32.eqz"]),
                Type::Integer => Some(wat!["i32.eq", Wat::const_i32(i32::MIN)]),
                Type::Float => Some(wat!["i32.eq", wat!["i32.reinterpret_f32"], wat!["i32.reinterpret_f32", wat!["f32.const", "nan"]]]),
                Type::String => Some(wat!["i32.eqz", Wat::call(ARRAY_LENGTH_FUNC_NAME, vec![])]),
                Type::Struct(_) => Some(wat!["i32.eq", Wat::const_i32(NULL_ADDR)]),
                Type::Null => Some(Wat::const_i32(1)),
                Type::Array(_) => Some(wat!["i32.eqz", Wat::call(ARRAY_LENGTH_FUNC_NAME, vec![])]),
                _ => None
            },
            UnaryOperatorToken::Plus => match operand_type {
                Type::Integer => Some(wat!["nop"]),
                Type::Float => Some(wat!["nop"]),
                _ => None
            },
            UnaryOperatorToken::Minus => match operand_type {
                Type::Integer => Some(wat!["i32.mul", Wat::const_i32(-1)]),
                Type::Float => Some(wat!["f32.neg"]),
                _ => None
            },
        };

        let output_type = match &self.token {
            UnaryOperatorToken::Not => Type::Boolean,
            UnaryOperatorToken::AsBool => Type::Boolean,
            UnaryOperatorToken::Plus => operand_type.clone(),
            UnaryOperatorToken::Minus => operand_type.clone(),
        };

        match wat_result {
            Some(wat) => Some(Wasm::typed(output_type, wat)),
            None => {
                context.error(&self, format!("unary operator {}: invalid operand type `{}`", &self.token, &operand_type));
                None
            }
        }
    }
}