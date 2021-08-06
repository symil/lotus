use crate::{items::UnaryOperator, program::BuiltinType};

use super::Type;

pub fn get_unary_operator_input_types(operator: &UnaryOperator) -> Vec<Type> {
    match operator {
        UnaryOperator::Not => vec![Type::Any(0)],
        UnaryOperator::Plus | UnaryOperator::Minus => vec![Type::builtin(BuiltinType::Integer)],
    }
}

pub fn get_unary_operator_output_type(operator: &UnaryOperator, input_type: &Type) -> Type {
    match operator {
        UnaryOperator::Not => Type::builtin(BuiltinType::Boolean),
        UnaryOperator::Plus | UnaryOperator::Minus => input_type.clone(),
    }
}