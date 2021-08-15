use crate::generation::{ToWatVec, Wat};
use super::{Type, VariableInfo};

#[derive(Default)]
pub struct Wasm {
    pub ty: Type,
    pub wat: Vec<Wat>,
    pub declared_variables: Vec<VariableInfo>,
}

impl Wasm {
    pub fn new<T : ToWatVec>(ty: Type, wat: T, declared_variables: Vec<VariableInfo>) -> Self {
        Self { ty, wat: wat.to_wat_vec(), declared_variables }
    }

    pub fn typed<T : ToWatVec>(ty: Type, wat: T) -> Self {
        Self {
            ty,
            wat: wat.to_wat_vec(),
            declared_variables: vec![]
        }
    }

    pub fn untyped<T : ToWatVec>(wat: T, declared_variables: Vec<VariableInfo>) -> Self {
        Self {
            ty: Type::Void,
            wat: wat.to_wat_vec(),
            declared_variables
        }
    }
}