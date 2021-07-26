use crate::items::{identifier::Identifier, struct_declaration::{ValueType, TypeSuffix}};

#[derive(Clone, Debug)]
pub enum ExpressionType {
    Void,
    Single(Identifier),
    Array(Identifier),
    Function(Vec<ExpressionType>, Option<Box<ExpressionType>>)
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