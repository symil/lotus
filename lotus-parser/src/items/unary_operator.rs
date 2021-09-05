use parsable::parsable;
use crate::{generation::{NULL_ADDR, ToWat, ToWatVec, Wat}, program::{ARRAY_GET_LENGTH_FUNC_NAME, ProgramContext, TypeOld, Wasm}, wat};

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
    pub fn process(&self, operand_type: &TypeOld, context: &mut ProgramContext) -> Option<Wasm> {
        let result = match &self.token {
            UnaryOperatorToken::AsBool => operand_type.to_bool(),
            UnaryOperatorToken::Not => match operand_type {
                TypeOld::Pointer(_) => Some(Wasm::simple(TypeOld::Boolean, wat!["i32.eqz"])),
                TypeOld::Boolean => Some(Wasm::simple(TypeOld::Boolean, wat!["i32.eqz"])),
                TypeOld::Integer => Some(Wasm::simple(TypeOld::Boolean, wat!["i32.eq", Wat::const_i32(i32::MIN)])),
                TypeOld::Float => Some(Wasm::simple(TypeOld::Boolean, wat!["i32.eq", wat!["i32.reinterpret_f32"], wat!["i32.reinterpret_f32", wat!["f32.const", "nan"]]])),
                TypeOld::String => Some(Wasm::simple(TypeOld::Boolean, wat!["i32.eqz", Wat::call(ARRAY_GET_LENGTH_FUNC_NAME, vec![])])),
                TypeOld::Struct(_) => Some(Wasm::simple(TypeOld::Boolean, wat!["i32.eq", Wat::const_i32(NULL_ADDR)])),
                TypeOld::Null => Some(Wasm::simple(TypeOld::Boolean, Wat::const_i32(1))),
                TypeOld::Array(_) => Some(Wasm::simple(TypeOld::Boolean, wat!["i32.eqz", Wat::call(ARRAY_GET_LENGTH_FUNC_NAME, vec![])])),
                _ => None
            },
            UnaryOperatorToken::Plus => match operand_type {
                TypeOld::Integer => Some(Wasm::empty(TypeOld::Integer)),
                TypeOld::Float => Some(Wasm::empty(TypeOld::Float)),
                _ => None
            },
            UnaryOperatorToken::Minus => match operand_type {
                TypeOld::Integer => Some(Wasm::simple(TypeOld::Integer, wat!["i32.mul", Wat::const_i32(-1)])),
                TypeOld::Float => Some(Wasm::simple(TypeOld::Float, wat!["f32.neg"])),
                _ => None
            },
        };

        match result {
            Some(wasm) => Some(wasm),
            None => {
                context.errors.add(&self, format!("unary operator {}: invalid operand type `{}`", &self.token, &operand_type));
                None
            }
        }
    }
}