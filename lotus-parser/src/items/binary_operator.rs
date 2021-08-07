use parsable::parsable;
use crate::{generation::{CONCAT_ARRAYS_FUNC_NAME, STRING_EQUAL_FUNC_NAME, ToWat, ToWatVec, Wat}, program::{ProgramContext, Type, Wasm}, wat};

#[parsable(impl_display=true)]
pub enum BinaryOperator {
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
        match self {
            Self::Mult | Self::Div | Self::Mod => 0,
            Self::Plus | Self::Minus => 1,
            Self::Eq | Self::Ne | Self::Ge | Self::Gt | Self::Le | Self::Lt => 2,
            Self::And => 3,
            Self::Or => 4,
        }
    }

    pub fn process(&self, operand_type: &Type, context: &mut ProgramContext) -> Option<Wasm> {
        match self {
            Self::Plus => match operand_type {
                Type::Pointer => Some(Wasm::typed(Type::Pointer, Wat::inst("i32.add"))),
                Type::Integer => Some(Wasm::typed(Type::Integer, Wat::inst("i32.add"))),
                Type::Float => Some(Wasm::typed(Type::Float, Wat::inst("f32.add"))),
                Type::String => Some(Wasm::typed(Type::String, Wat::call(CONCAT_ARRAYS_FUNC_NAME, vec![]))),
                Type::Array(item_type) => Some(Wasm::typed(Type::Array(item_type.clone()), Wat::call(CONCAT_ARRAYS_FUNC_NAME, vec![]))),
                _ => None
            },
            Self::Minus => match operand_type {
                Type::Pointer => Some(Wasm::typed(Type::Pointer, Wat::inst("i32.sub"))),
                Type::Integer => Some(Wasm::typed(Type::Integer, Wat::inst("i32.sub"))),
                Type::Float => Some(Wasm::typed(Type::Float, Wat::inst("f32.sub"))),
                _ => None
            },
            Self::Mult => match operand_type {
                Type::Integer => Some(Wasm::typed(Type::Integer, Wat::inst("i32.mul"))),
                Type::Float => Some(Wasm::typed(Type::Float, Wat::inst("f32.mul"))),
                _ => None
            },
            Self::Div => match operand_type {
                Type::Integer => Some(Wasm::typed(Type::Integer, Wat::inst("i32.div_s"))),
                Type::Float => Some(Wasm::typed(Type::Float, Wat::inst("f32.div"))),
                _ => None
            },
            Self::Mod => match operand_type {
                Type::Integer => Some(Wasm::typed(Type::Integer, Wat::inst("i32.rem_s"))),
                _ => None
            },
            Self::And => match operand_type {
                Type::Boolean => Some(Wasm::typed(Type::Boolean, Wat::inst("i32.and"))),
                _ => None
            },
            Self::Or => match operand_type {
                Type::Boolean => Some(Wasm::typed(Type::Boolean, Wat::inst("i32.or"))),
                _ => None
            },
            Self::Eq => match operand_type {
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
            Self::Ne => match operand_type {
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
            Self::Ge => match operand_type {
                Type::Integer => Some(Wasm::typed(Type::Boolean, Wat::inst("i32.ge"))),
                Type::Float => Some(Wasm::typed(Type::Boolean, Wat::inst("f32.ge"))),
                _ => None
            },
            Self::Gt => match operand_type {
                Type::Integer => Some(Wasm::typed(Type::Boolean, Wat::inst("i32.gt"))),
                Type::Float => Some(Wasm::typed(Type::Boolean, Wat::inst("f32.gt"))),
                _ => None
            },
            Self::Le => match operand_type {
                Type::Integer => Some(Wasm::typed(Type::Boolean, Wat::inst("i32.le"))),
                Type::Float => Some(Wasm::typed(Type::Boolean, Wat::inst("f32.le"))),
                _ => None
            },
            Self::Lt => match operand_type {
                Type::Integer => Some(Wasm::typed(Type::Boolean, Wat::inst("i32.lt"))),
                Type::Float => Some(Wasm::typed(Type::Boolean, Wat::inst("f32.lt"))),
                _ => None
            },
        }
    }
}