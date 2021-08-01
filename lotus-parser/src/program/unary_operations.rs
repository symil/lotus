use crate::{items::UnaryOperator, program::BuiltinType};

use super::ExpressionType;

pub fn get_unary_operator_input_types(operator: &UnaryOperator) -> Vec<ExpressionType> {
    match operator {
        UnaryOperator::Not => vec![ExpressionType::Anonymous(0)],
        UnaryOperator::Plus | UnaryOperator::Minus => vec![ExpressionType::builtin(BuiltinType::Number)],
    }
}

pub fn get_unary_operator_output_type(operator: &UnaryOperator, input_type: &ExpressionType) -> ExpressionType {
    match operator {
        UnaryOperator::Not => ExpressionType::builtin(BuiltinType::Boolean),
        UnaryOperator::Plus | UnaryOperator::Minus => input_type.clone(),
    }
}