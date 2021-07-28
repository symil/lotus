use std::fmt;

use crate::items::{identifier::Identifier, struct_declaration::{ValueType, TypeSuffix}};

#[derive(Clone, Debug)]
pub enum ExpressionType {
    Void,
    Single(Identifier),
    SingleAny(u32),
    Array(Identifier),
    ArrayAny(u32),
    Function(Vec<ExpressionType>, Box<ExpressionType>)
}

impl ExpressionType {
    pub fn void() -> Self {
        ExpressionType::Void
    }

    pub fn from_value_type(value_type: &ValueType) -> Self {
        let name = value_type.name.clone();

        match value_type.suffix {
            Some(TypeSuffix::Array) => Self::Array(name),
            None => Self::Single(name),
        }
    }

    pub fn is_index(&self) -> bool {
        match self {
            ExpressionType::Single(name) => name.is("num"),
            _ => false
        }
    }

    pub fn is_void(&self) -> bool {
        match self {
            ExpressionType::Void => true,
            _ => false
        }
    }
}

impl Default for ExpressionType {
    fn default() -> Self {
        ExpressionType::Void
    }
}

impl fmt::Display for ExpressionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExpressionType::Void => write!(f, "<void>"),
            ExpressionType::Single(type_name) => write!(f, "{}", type_name),
            ExpressionType::SingleAny(id) => write!(f, "<any.{}>", id),
            ExpressionType::Array(type_name) => write!(f, "{}[]", type_name),
            ExpressionType::ArrayAny(id) => write!(f, "<any.{}>[]", id),
            ExpressionType::Function(arguments, return_type) => {
                let args_joined = arguments.iter().map(|arg| format!("{}", arg)).collect::<Vec<String>>().join(",");
                let return_type_str = match Box::as_ref(return_type) {
                    ExpressionType::Void => String::new(),
                    _ => format!(" -> {}", return_type)
                };

                write!(f, "fn({}){}", args_joined, return_type_str)
            },
        }
    }
}