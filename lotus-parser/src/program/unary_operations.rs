use crate::{items::expression::UnaryOperator, program::expression_type::BuiltinType};

use super::expression_type::ExpressionType;

pub fn get_unary_operator_input_types(operator: &UnaryOperator) -> Vec<ExpressionType> {
    match operator {
        UnaryOperator::Not => vec![ExpressionType::Anonymous(0)],
        UnaryOperator::Plus | UnaryOperator::Minus => vec![ExpressionType::single_builtin(BuiltinType::Number)],
    }
}

pub fn get_unary_operator_output_type(operator: &UnaryOperator, input_type: &ExpressionType) -> ExpressionType {
    match operator {
        UnaryOperator::Not => ExpressionType::single_builtin(BuiltinType::Boolean),
        UnaryOperator::Plus | UnaryOperator::Minus => input_type.clone(),
    }
}