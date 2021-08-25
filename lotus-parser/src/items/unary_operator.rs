use parsable::parsable;
use crate::{generation::{NULL_ADDR, ToWat, ToWatVec, Wat}, program::{ARRAY_GET_LENGTH_FUNC_NAME, ProgramContext, Type, Wasm}, wat};

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
        let result = match &self.token {
            UnaryOperatorToken::AsBool => operand_type.to_bool(),
            UnaryOperatorToken::Not => match operand_type {
                Type::Pointer(_) => Some(Wasm::simple(Type::Boolean, wat!["i32.eqz"])),
                Type::Boolean => Some(Wasm::simple(Type::Boolean, wat!["i32.eqz"])),
                Type::Integer => Some(Wasm::simple(Type::Boolean, wat!["i32.eq", Wat::const_i32(i32::MIN)])),
                Type::Float => Some(Wasm::simple(Type::Boolean, wat!["i32.eq", wat!["i32.reinterpret_f32"], wat!["i32.reinterpret_f32", wat!["f32.const", "nan"]]])),
                Type::String => Some(Wasm::simple(Type::Boolean, wat!["i32.eqz", Wat::call(ARRAY_GET_LENGTH_FUNC_NAME, vec![])])),
                Type::Struct(_) => Some(Wasm::simple(Type::Boolean, wat!["i32.eq", Wat::const_i32(NULL_ADDR)])),
                Type::Null => Some(Wasm::simple(Type::Boolean, Wat::const_i32(1))),
                Type::Array(_) => Some(Wasm::simple(Type::Boolean, wat!["i32.eqz", Wat::call(ARRAY_GET_LENGTH_FUNC_NAME, vec![])])),
                _ => None
            },
            UnaryOperatorToken::Plus => match operand_type {
                Type::Integer => Some(Wasm::empty(Type::Integer)),
                Type::Float => Some(Wasm::empty(Type::Float)),
                _ => None
            },
            UnaryOperatorToken::Minus => match operand_type {
                Type::Integer => Some(Wasm::simple(Type::Integer, wat!["i32.mul", Wat::const_i32(-1)])),
                Type::Float => Some(Wasm::simple(Type::Float, wat!["f32.neg"])),
                _ => None
            },
        };

        match result {
            Some(wasm) => Some(wasm),
            None => {
                context.error(&self, format!("unary operator {}: invalid operand type `{}`", &self.token, &operand_type));
                None
            }
        }
    }
}