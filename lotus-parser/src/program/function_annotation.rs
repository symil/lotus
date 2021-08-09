use crate::items::Identifier;
use super::Type;

#[derive(Clone)]
pub struct FunctionAnnotation {
    pub wasm_name: String,
    pub arguments: Vec<(Identifier, Type)>,
    pub return_type: Type
}

impl FunctionAnnotation {
    pub fn new(wasm_name: String) -> Self {
        Self {
            wasm_name: wasm_name,
            arguments: vec![],
            return_type: Type::Void
        }
    }

    pub fn get_type(&self) -> Type {
        let arguments = self.arguments.iter().map(|(_, arg)| arg.clone()).collect();
        let return_type = self.return_type.clone();

        Type::function(arguments, return_type)
    }
}