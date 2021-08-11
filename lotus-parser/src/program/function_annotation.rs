use crate::{generation::Wat, items::Identifier};
use super::{Type, Wasm, WithId};

#[derive(Default, Debug)]
pub struct FunctionAnnotation {
    pub index: usize,
    pub wasm_name: String,
    pub this_type: Option<Type>,
    pub payload_type: Option<Type>,
    pub arguments: Vec<(Identifier, Type)>,
    pub return_type: Type,
    pub wat: Wat
}

impl FunctionAnnotation {
    pub fn get_type(&self) -> Type {
        let arguments = self.arguments.iter().map(|(_, arg)| arg.clone()).collect();
        let return_type = self.return_type.clone();

        Type::function(arguments, return_type)
    }
}

impl WithId for FunctionAnnotation {
    fn get_id(&self) -> usize {
        self.index
    }
}