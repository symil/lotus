use crate::items::identifier::Identifier;

use super::expression_type::ExpressionType;

pub struct FunctionDefinition {
    pub name: Identifier,
    pub arguments: Vec<ExpressionType>,
    pub return_type: Option<ExpressionType>
}