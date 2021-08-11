use parsable::parsable;
use crate::{generation::{ARRAY_CONCAT_FUNC_NAME, STRING_EQUAL_FUNC_NAME, ToWat, ToWatVec, Wat}, program::{ProgramContext, Type, Wasm}, wat};

#[parsable]
#[derive(Default)]
pub struct BinaryOperator {
    pub token: BinaryOperatorToken
}

#[parsable(impl_display=true)]
pub enum BinaryOperatorToken {
    Plus = "+",
    Minus = "-",
    Mult = "*",
    Div = "/",
    Mod = "%",
    And = "&&",
    Or = "||",
    Eq = "==",
    Ne = "!=",
    Ge = ">=",
    Gt = ">",
    Le = "<=",
    Lt = "<",
}

impl BinaryOperator {
    pub fn get_priority(&self) -> usize {
        match &self.token {
            BinaryOperatorToken::Mult | BinaryOperatorToken::Div | BinaryOperatorToken::Mod => 0,
            BinaryOperatorToken::Plus | BinaryOperatorToken::Minus => 1,
            BinaryOperatorToken::Eq | BinaryOperatorToken::Ne | BinaryOperatorToken::Ge | BinaryOperatorToken::Gt | BinaryOperatorToken::Le | BinaryOperatorToken::Lt => 2,
            BinaryOperatorToken::And => 3,
            BinaryOperatorToken::Or => 4,
        }
    }

    pub fn process(&self, operand_type: &Type, context: &mut ProgramContext) -> Option<Wasm> {
        match &self.token {
            BinaryOperatorToken::Plus => match operand_type {
                Type::Pointer => Some(Wasm::typed(Type::Pointer, Wat::inst("i32.add"))),
                Type::Integer => Some(Wasm::typed(Type::Integer, Wat::inst("i32.add"))),
                Type::Float => Some(Wasm::typed(Type::Float, Wat::inst("f32.add"))),
                Type::String => Some(Wasm::typed(Type::String, Wat::call(ARRAY_CONCAT_FUNC_NAME, vec![]))),
                Type::Array(item_type) => Some(Wasm::typed(Type::Array(item_type.clone()), Wat::call(ARRAY_CONCAT_FUNC_NAME, vec![]))),
                _ => None
            },
            BinaryOperatorToken::Minus => match operand_type {
                Type::Pointer => Some(Wasm::typed(Type::Pointer, Wat::inst("i32.sub"))),
                Type::Integer => Some(Wasm::typed(Type::Integer, Wat::inst("i32.sub"))),
                Type::Float => Some(Wasm::typed(Type::Float, Wat::inst("f32.sub"))),
                _ => None
            },
            BinaryOperatorToken::Mult => match operand_type {
                Type::Integer => Some(Wasm::typed(Type::Integer, Wat::inst("i32.mul"))),
                Type::Float => Some(Wasm::typed(Type::Float, Wat::inst("f32.mul"))),
                _ => None
            },
            BinaryOperatorToken::Div => match operand_type {
                Type::Integer => Some(Wasm::typed(Type::Integer, Wat::inst("i32.div_s"))),
                Type::Float => Some(Wasm::typed(Type::Float, Wat::inst("f32.div"))),
                _ => None
            },
            BinaryOperatorToken::Mod => match operand_type {
                Type::Integer => Some(Wasm::typed(Type::Integer, Wat::inst("i32.rem_s"))),
                _ => None
            },
            BinaryOperatorToken::And => match operand_type {
                Type::Boolean => Some(Wasm::typed(Type::Boolean, Wat::inst("i32.and"))),
                _ => None
            },
            BinaryOperatorToken::Or => match operand_type {
                Type::Boolean => Some(Wasm::typed(Type::Boolean, Wat::inst("i32.or"))),
                _ => None
            },
            BinaryOperatorToken::Eq => match operand_type {
                Type::Pointer => Some(Wasm::typed(Type::Boolean, Wat::inst("i32.eq"))),
                Type::Boolean => Some(Wasm::typed(Type::Boolean, Wat::inst("i32.eq"))),
                Type::Integer => Some(Wasm::typed(Type::Boolean, Wat::inst("i32.eq"))),
                Type::Float => Some(Wasm::typed(Type::Boolean, Wat::inst("f32.eq"))),
                Type::String => Some(Wasm::typed(Type::Boolean, Wat::call(STRING_EQUAL_FUNC_NAME, vec![]))),
                Type::Null => Some(Wasm::typed(Type::Boolean, Wat::inst("i32.eq"))),
                Type::Struct(_) => Some(Wasm::typed(Type::Boolean, Wat::inst("i32.eq"))),
                Type::Array(_) => Some(Wasm::typed(Type::Boolean, Wat::inst("i32.eq"))),
                _ => None
            },
            BinaryOperatorToken::Ne => match operand_type {
                Type::Pointer => Some(Wasm::typed(Type::Boolean, Wat::inst("i32.ne"))),
                Type::Boolean => Some(Wasm::typed(Type::Boolean, Wat::inst("i32.ne"))),
                Type::Integer => Some(Wasm::typed(Type::Boolean, Wat::inst("i32.ne"))),
                Type::Float => Some(Wasm::typed(Type::Boolean, Wat::inst("f32.ne"))),
                Type::String => Some(Wasm::typed(Type::Boolean, wat!["i32.eqz", Wat::call(STRING_EQUAL_FUNC_NAME, vec![])])),
                Type::Null => Some(Wasm::typed(Type::Boolean, Wat::inst("i32.ne"))),
                Type::Struct(_) => Some(Wasm::typed(Type::Boolean, Wat::inst("i32.ne"))),
                Type::Array(_) => Some(Wasm::typed(Type::Boolean, Wat::inst("i32.ne"))),
                _ => None
            },
            BinaryOperatorToken::Ge => match operand_type {
                Type::Integer => Some(Wasm::typed(Type::Boolean, Wat::inst("i32.ge"))),
                Type::Float => Some(Wasm::typed(Type::Boolean, Wat::inst("f32.ge"))),
                _ => None
            },
            BinaryOperatorToken::Gt => match operand_type {
                Type::Integer => Some(Wasm::typed(Type::Boolean, Wat::inst("i32.gt"))),
                Type::Float => Some(Wasm::typed(Type::Boolean, Wat::inst("f32.gt"))),
                _ => None
            },
            BinaryOperatorToken::Le => match operand_type {
                Type::Integer => Some(Wasm::typed(Type::Boolean, Wat::inst("i32.le"))),
                Type::Float => Some(Wasm::typed(Type::Boolean, Wat::inst("f32.le"))),
                _ => None
            },
            BinaryOperatorToken::Lt => match operand_type {
                Type::Integer => Some(Wasm::typed(Type::Boolean, Wat::inst("i32.lt"))),
                Type::Float => Some(Wasm::typed(Type::Boolean, Wat::inst("f32.lt"))),
                _ => None
            },
        }
    }
}

impl Default for BinaryOperatorToken {
    fn default() -> Self {
        Self::Plus
    }
}