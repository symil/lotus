use crate::items::Identifier;

use super::ExpressionType;

#[derive(Clone)]
pub struct FunctionAnnotation {
    pub name: Identifier,
    pub arguments: Vec<(Identifier, ExpressionType)>,
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
        let arguments = self.arguments.iter().map(|(_, arg)| arg.clone()).collect();
        let return_type = self.return_type.clone();

        ExpressionType::function(arguments, return_type)
    }
}