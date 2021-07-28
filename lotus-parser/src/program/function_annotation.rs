use crate::items::identifier::Identifier;

use super::expression_type::ExpressionType;

pub struct FunctionAnnotation {
    pub name: Identifier,
    pub arguments: Vec<ExpressionType>,
    pub return_type: ExpressionType
}

impl FunctionAnnotation {
    pub fn new(name: &Identifier) -> Self {
        Self {
            name: name.clone(),
            arguments: vec![],
            return_type: ExpressionType::Void
        }
    }

    pub fn get_expr_type(&self) -> ExpressionType {
        ExpressionType::function(self.arguments.clone(), self.return_type.clone())
    }
}