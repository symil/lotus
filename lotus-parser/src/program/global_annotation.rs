use crate::generation::Wat;
use super::{Type, WithId};

#[derive(Default, Debug)]
pub struct GlobalAnnotation {
    pub index: usize,
    pub wasm_name: String,
    pub ty: Type,
    pub value: Vec<Wat>
}

impl WithId for GlobalAnnotation {
    fn get_id(&self) -> usize {
        self.index
    }
}