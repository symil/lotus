use crate::generation::{ToWatVec, Wat};
use super::{Type, VariableInfo};

#[derive(Default, Debug)]
pub struct Wasm {
    pub ty: Type,
    pub wat: Vec<Wat>,
    pub variables: Vec<VariableInfo>,
}

impl Wasm {
    pub fn new<T : ToWatVec>(ty: Type, wat: T, variables: Vec<VariableInfo>) -> Self {
        Self {
            ty,
            wat: wat.to_wat_vec(),
            variables
        }
    }

    pub fn merge(ty: Type, source: Vec<Self>) -> Self {
        let mut wat = vec![];
        let mut variables = vec![];

        for wasm in source {
            wat.extend(wasm.wat);
            variables.extend(wasm.variables);
        }

        Self { ty, wat, variables }
    }

    pub fn simple(ty: Type, wat: Wat) -> Self {
        Self {
            ty,
            wat: wat.to_wat_vec(),
            variables: vec![]
        }
    }

    pub fn empty(ty: Type) -> Self {
        Self {
            ty,
            wat: vec![],
            variables: vec![]
        }
    }
}