use parsable::parsable;
use crate::{generation::{ARRAY_CONCAT_FUNC_NAME, Wat}, program::{ProgramContext, STRING_CONCAT_FUNC_NAME, STRING_EQUAL_FUNC_NAME, Type, Wasm}, wat};

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
    Shl = "<<",
    Shr = ">>",
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
        self.token.get_priority()
    }

    pub fn process(&self, operand_type: &Type, context: &mut ProgramContext) -> Option<Wasm> {
        self.token.process(operand_type, context)
    }
}

impl BinaryOperatorToken {
    pub fn get_priority(&self) -> usize {
        match self {
            Self::Mult | Self::Div | Self::Mod => 1,
            Self::Plus | Self::Minus => 2,
            Self::Shl | Self::Shr => 3,
            Self::Eq | Self::Ne | Self::Ge | Self::Gt | Self::Le | Self::Lt => 4,
            Self::And => 5,
            Self::Or => 6,
        }
    }

    pub fn process(&self, operand_type: &Type, context: &mut ProgramContext) -> Option<Wasm> {
        match self {
            Self::Plus => match operand_type {
                Type::Pointer(pointed_type) => Some(Wasm::simple(Type::Pointer(pointed_type.clone()), Wat::inst("i32.add"))),
                Type::Integer => Some(Wasm::simple(Type::Integer, Wat::inst("i32.add"))),
                Type::Float => Some(Wasm::simple(Type::Float, Wat::inst("f32.add"))),
                Type::String => Some(Wasm::simple(Type::String, Wat::call(STRING_CONCAT_FUNC_NAME, vec![]))),
                Type::Array(item_type) => Some(Wasm::simple(Type::Array(item_type.clone()), Wat::call(ARRAY_CONCAT_FUNC_NAME, vec![]))),
                _ => None
            },
            Self::Minus => match operand_type {
                Type::Pointer(pointed_type) => Some(Wasm::simple(Type::Pointer(pointed_type.clone()), Wat::inst("i32.sub"))),
                Type::Integer => Some(Wasm::simple(Type::Integer, Wat::inst("i32.sub"))),
                Type::Float => Some(Wasm::simple(Type::Float, Wat::inst("f32.sub"))),
                _ => None
            },
            Self::Mult => match operand_type {
                Type::Integer => Some(Wasm::simple(Type::Integer, Wat::inst("i32.mul"))),
                Type::Float => Some(Wasm::simple(Type::Float, Wat::inst("f32.mul"))),
                _ => None
            },
            Self::Div => match operand_type {
                Type::Integer => Some(Wasm::simple(Type::Integer, Wat::inst("i32.div_s"))),
                Type::Float => Some(Wasm::simple(Type::Float, Wat::inst("f32.div"))),
                _ => None
            },
            Self::Mod => match operand_type {
                Type::Integer => Some(Wasm::simple(Type::Integer, Wat::inst("i32.rem_s"))),
                _ => None
            },
            Self::Shl => match operand_type {
                Type::Integer => Some(Wasm::simple(Type::Integer, Wat::inst("i32.shl"))),
                _ => None
            },
            Self::Shr => match operand_type {
                Type::Integer => Some(Wasm::simple(Type::Integer, Wat::inst("i32.shr_u"))),
                _ => None
            },
            Self::And => match operand_type {
                Type::Boolean => Some(Wasm::simple(Type::Boolean, Wat::inst("i32.and"))),
                _ => None
            },
            Self::Or => match operand_type {
                Type::Boolean => Some(Wasm::simple(Type::Boolean, Wat::inst("i32.or"))),
                _ => None
            },
            Self::Eq => match operand_type {
                Type::Pointer(_) => Some(Wasm::simple(Type::Boolean, Wat::inst("i32.eq"))),
                Type::Boolean => Some(Wasm::simple(Type::Boolean, Wat::inst("i32.eq"))),
                Type::Integer => Some(Wasm::simple(Type::Boolean, Wat::inst("i32.eq"))),
                Type::Float => Some(Wasm::simple(Type::Boolean, Wat::inst("f32.eq"))),
                Type::String => Some(Wasm::simple(Type::Boolean, Wat::call(STRING_EQUAL_FUNC_NAME, vec![]))),
                Type::Null => Some(Wasm::simple(Type::Boolean, Wat::inst("i32.eq"))),
                Type::Struct(_) => Some(Wasm::simple(Type::Boolean, Wat::inst("i32.eq"))),
                Type::Array(_) => Some(Wasm::simple(Type::Boolean, Wat::inst("i32.eq"))),
                _ => None
            },
            Self::Ne => match operand_type {
                Type::Pointer(_) => Some(Wasm::simple(Type::Boolean, Wat::inst("i32.ne"))),
                Type::Boolean => Some(Wasm::simple(Type::Boolean, Wat::inst("i32.ne"))),
                Type::Integer => Some(Wasm::simple(Type::Boolean, Wat::inst("i32.ne"))),
                Type::Float => Some(Wasm::simple(Type::Boolean, Wat::inst("f32.ne"))),
                Type::String => Some(Wasm::simple(Type::Boolean, wat!["i32.eqz", Wat::call(STRING_EQUAL_FUNC_NAME, vec![])])),
                Type::Null => Some(Wasm::simple(Type::Boolean, Wat::inst("i32.ne"))),
                Type::Struct(_) => Some(Wasm::simple(Type::Boolean, Wat::inst("i32.ne"))),
                Type::Array(_) => Some(Wasm::simple(Type::Boolean, Wat::inst("i32.ne"))),
                _ => None
            },
            Self::Ge => match operand_type {
                Type::Integer => Some(Wasm::simple(Type::Boolean, Wat::inst("i32.ge_s"))),
                Type::Float => Some(Wasm::simple(Type::Boolean, Wat::inst("f32.ge"))),
                _ => None
            },
            Self::Gt => match operand_type {
                Type::Integer => Some(Wasm::simple(Type::Boolean, Wat::inst("i32.gt_s"))),
                Type::Float => Some(Wasm::simple(Type::Boolean, Wat::inst("f32.gt"))),
                _ => None
            },
            Self::Le => match operand_type {
                Type::Integer => Some(Wasm::simple(Type::Boolean, Wat::inst("i32.le_s"))),
                Type::Float => Some(Wasm::simple(Type::Boolean, Wat::inst("f32.le"))),
                _ => None
            },
            Self::Lt => match operand_type {
                Type::Integer => Some(Wasm::simple(Type::Boolean, Wat::inst("i32.lt_s"))),
                Type::Float => Some(Wasm::simple(Type::Boolean, Wat::inst("f32.lt"))),
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