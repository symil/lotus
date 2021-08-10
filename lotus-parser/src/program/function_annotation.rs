use crate::items::Identifier;
use super::{Type, Wasm};

#[derive(Default)]
pub struct FunctionAnnotation {
    pub wasm_name: String,
    pub this_type: Option<Type>,
    pub payload_type: Option<Type>,
    pub arguments: Vec<(Identifier, Type)>,
    pub return_type: Type,
    pub body: Wasm
}

impl FunctionAnnotation {
    pub fn get_type(&self) -> Type {
        let arguments = self.arguments.iter().map(|(_, arg)| arg.clone()).collect();
        let return_type = self.return_type.clone();

        Type::function(arguments, return_type)
    }
}