use crate::generation::{ToWatVec, Wat};
use super::{TypeOld, VariableInfo};

#[derive(Default, Debug)]
pub struct Wasm {
    pub ty: TypeOld,
    pub wat: Vec<Wat>,
    pub variables: Vec<VariableInfo>,
}

impl Wasm {
    pub fn new<T : ToWatVec>(ty: TypeOld, wat: T, variables: Vec<VariableInfo>) -> Self {
        Self {
            ty,
            wat: wat.to_wat_vec(),
            variables
        }
    }

    pub fn merge(ty: TypeOld, source: Vec<Self>) -> Self {
        let mut wat = vec![];
        let mut variables = vec![];

        for wasm in source {
            wat.extend(wasm.wat);
            variables.extend(wasm.variables);
        }

        Self { ty, wat, variables }
    }

    pub fn simple(ty: TypeOld, wat: Wat) -> Self {
        Self {
            ty,
            wat: wat.to_wat_vec(),
            variables: vec![]
        }
    }

    pub fn empty(ty: TypeOld) -> Self {
        Self {
            ty,
            wat: vec![],
            variables: vec![]
        }
    }

    pub fn from_wat<T : ToWatVec>(wat: T) -> Self {
        Self {
            ty: TypeOld::Void,
            wat: wat.to_wat_vec(),
            variables: vec![]
        }
    }
}