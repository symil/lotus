use crate::generation::{ToWatVec, Wat};
use super::Type;

pub struct Wasm {
    pub ty: Type,
    pub wat: Vec<Wat>
}

impl Wasm {
    pub fn typed<T : ToWatVec>(ty: Type, wat: T) -> Self {
        Self {
            ty,
            wat: wat.to_wat_vec()
        }
    }

    pub fn untyped<T : ToWatVec>(wat: T) -> Self {
        Self {
            ty: Type::Void,
            wat: wat.to_wat_vec()
        }
    }
}