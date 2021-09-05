use parsable::{DataLocation, parsable};
use crate::{generation::{Wat}, items::Identifier, program::{ARRAY_CONCAT_FUNC_NAME, ProgramContext, STRING_CONCAT_FUNC_NAME, STRING_EQUAL_FUNC_NAME, TypeOld, VariableInfo, VariableKind, Wasm}, wat};

#[parsable]
#[derive(Default)]
pub struct BinaryOperator {
    pub token: BinaryOperatorToken
}

#[parsable(impl_display=true)]
#[derive(PartialEq)]
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
    SingleAnd = "&",
    SingleOr = "|",
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

    pub fn get_short_circuit_wasm(&self) -> Option<Wasm> {
        self.token.get_short_circuit_wasm(self)
    }

    pub fn process(&self, operand_type: &TypeOld, context: &mut ProgramContext) -> Option<Wasm> {
        self.token.process(operand_type, context)
    }
}

impl BinaryOperatorToken {
    pub fn get_priority(&self) -> usize {
        match self {
            Self::Mult | Self::Div | Self::Mod => 1,
            Self::Plus | Self::Minus => 2,
            Self::Shl | Self::Shr => 3,
            Self::SingleAnd => 4,
            Self::SingleOr => 5,
            Self::Eq | Self::Ne | Self::Ge | Self::Gt | Self::Le | Self::Lt => 6,
            Self::And => 7,
            Self::Or => 8,
        }
    }

    pub fn get_short_circuit_wasm(&self, location: &DataLocation) -> Option<Wasm> {
        match self {
            Self::And | Self::Or => {
                let tmp_var_name = Identifier::unique("tmp", location).to_unique_string();
                let tmp_var_info = VariableInfo::new(tmp_var_name.clone(), TypeOld::Boolean, VariableKind::Local);
                let branch = if self == &Self::And {
                    wat!["br_if", 0, wat!["i32.eqz"]]
                } else {
                    wat!["br_if", 0]
                };
                let wat = vec![
                    Wat::tee_local(&tmp_var_name),
                    Wat::get_local(&tmp_var_name),
                    branch
                ];

                Some(Wasm::new(TypeOld::Boolean, wat, vec![tmp_var_info]))
            }
            _ => None
        }
    }

    pub fn process(&self, operand_type: &TypeOld, context: &mut ProgramContext) -> Option<Wasm> {
        match self {
            Self::Plus => match operand_type {
                TypeOld::Pointer(pointed_type) => Some(Wasm::simple(TypeOld::Pointer(pointed_type.clone()), Wat::inst("i32.add"))),
                TypeOld::Integer => Some(Wasm::simple(TypeOld::Integer, Wat::inst("i32.add"))),
                TypeOld::Float => Some(Wasm::simple(TypeOld::Float, Wat::inst("f32.add"))),
                TypeOld::String => Some(Wasm::simple(TypeOld::String, Wat::call(STRING_CONCAT_FUNC_NAME, vec![]))),
                TypeOld::Array(item_type) => Some(Wasm::simple(TypeOld::Array(item_type.clone()), Wat::call(ARRAY_CONCAT_FUNC_NAME, vec![]))),
                _ => None
            },
            Self::Minus => match operand_type {
                TypeOld::Pointer(pointed_type) => Some(Wasm::simple(TypeOld::Pointer(pointed_type.clone()), Wat::inst("i32.sub"))),
                TypeOld::Integer => Some(Wasm::simple(TypeOld::Integer, Wat::inst("i32.sub"))),
                TypeOld::Float => Some(Wasm::simple(TypeOld::Float, Wat::inst("f32.sub"))),
                _ => None
            },
            Self::Mult => match operand_type {
                TypeOld::Integer => Some(Wasm::simple(TypeOld::Integer, Wat::inst("i32.mul"))),
                TypeOld::Float => Some(Wasm::simple(TypeOld::Float, Wat::inst("f32.mul"))),
                _ => None
            },
            Self::Div => match operand_type {
                TypeOld::Integer => Some(Wasm::simple(TypeOld::Integer, Wat::inst("i32.div_s"))),
                TypeOld::Float => Some(Wasm::simple(TypeOld::Float, Wat::inst("f32.div"))),
                _ => None
            },
            Self::Mod => match operand_type {
                TypeOld::Integer => Some(Wasm::simple(TypeOld::Integer, Wat::inst("i32.rem_s"))),
                _ => None
            },
            Self::Shl => match operand_type {
                TypeOld::Integer => Some(Wasm::simple(TypeOld::Integer, Wat::inst("i32.shl"))),
                _ => None
            },
            Self::Shr => match operand_type {
                TypeOld::Integer => Some(Wasm::simple(TypeOld::Integer, Wat::inst("i32.shr_u"))),
                _ => None
            },
            Self::And => match operand_type {
                TypeOld::Boolean => Some(Wasm::simple(TypeOld::Boolean, Wat::inst("i32.and"))),
                _ => None
            },
            Self::Or => match operand_type {
                TypeOld::Boolean => Some(Wasm::simple(TypeOld::Boolean, Wat::inst("i32.or"))),
                _ => None
            },
            Self::SingleAnd => match operand_type {
                TypeOld::Boolean => Some(Wasm::simple(TypeOld::Boolean, Wat::inst("i32.and"))),
                TypeOld::Integer => Some(Wasm::simple(TypeOld::Integer, Wat::inst("i32.and"))),
                _ => None
            },
            Self::SingleOr => match operand_type {
                TypeOld::Boolean => Some(Wasm::simple(TypeOld::Boolean, Wat::inst("i32.or"))),
                TypeOld::Integer => Some(Wasm::simple(TypeOld::Integer, Wat::inst("i32.or"))),
                _ => None
            },
            Self::Eq => match operand_type {
                TypeOld::Pointer(_) => Some(Wasm::simple(TypeOld::Boolean, Wat::inst("i32.eq"))),
                TypeOld::Boolean => Some(Wasm::simple(TypeOld::Boolean, Wat::inst("i32.eq"))),
                TypeOld::Integer => Some(Wasm::simple(TypeOld::Boolean, Wat::inst("i32.eq"))),
                TypeOld::Float => Some(Wasm::simple(TypeOld::Boolean, Wat::inst("f32.eq"))),
                TypeOld::String => Some(Wasm::simple(TypeOld::Boolean, Wat::call(STRING_EQUAL_FUNC_NAME, vec![]))),
                TypeOld::Null => Some(Wasm::simple(TypeOld::Boolean, Wat::inst("i32.eq"))),
                TypeOld::Struct(_) => Some(Wasm::simple(TypeOld::Boolean, Wat::inst("i32.eq"))),
                TypeOld::Array(_) => Some(Wasm::simple(TypeOld::Boolean, Wat::inst("i32.eq"))),
                _ => None
            },
            Self::Ne => match operand_type {
                TypeOld::Pointer(_) => Some(Wasm::simple(TypeOld::Boolean, Wat::inst("i32.ne"))),
                TypeOld::Boolean => Some(Wasm::simple(TypeOld::Boolean, Wat::inst("i32.ne"))),
                TypeOld::Integer => Some(Wasm::simple(TypeOld::Boolean, Wat::inst("i32.ne"))),
                TypeOld::Float => Some(Wasm::simple(TypeOld::Boolean, Wat::inst("f32.ne"))),
                TypeOld::String => Some(Wasm::simple(TypeOld::Boolean, wat!["i32.eqz", Wat::call(STRING_EQUAL_FUNC_NAME, vec![])])),
                TypeOld::Null => Some(Wasm::simple(TypeOld::Boolean, Wat::inst("i32.ne"))),
                TypeOld::Struct(_) => Some(Wasm::simple(TypeOld::Boolean, Wat::inst("i32.ne"))),
                TypeOld::Array(_) => Some(Wasm::simple(TypeOld::Boolean, Wat::inst("i32.ne"))),
                _ => None
            },
            Self::Ge => match operand_type {
                TypeOld::Integer => Some(Wasm::simple(TypeOld::Boolean, Wat::inst("i32.ge_s"))),
                TypeOld::Float => Some(Wasm::simple(TypeOld::Boolean, Wat::inst("f32.ge"))),
                _ => None
            },
            Self::Gt => match operand_type {
                TypeOld::Integer => Some(Wasm::simple(TypeOld::Boolean, Wat::inst("i32.gt_s"))),
                TypeOld::Float => Some(Wasm::simple(TypeOld::Boolean, Wat::inst("f32.gt"))),
                _ => None
            },
            Self::Le => match operand_type {
                TypeOld::Integer => Some(Wasm::simple(TypeOld::Boolean, Wat::inst("i32.le_s"))),
                TypeOld::Float => Some(Wasm::simple(TypeOld::Boolean, Wat::inst("f32.le"))),
                _ => None
            },
            Self::Lt => match operand_type {
                TypeOld::Integer => Some(Wasm::simple(TypeOld::Boolean, Wat::inst("i32.lt_s"))),
                TypeOld::Float => Some(Wasm::simple(TypeOld::Boolean, Wat::inst("f32.lt"))),
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