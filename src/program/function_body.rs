use super::{Vasm, Wat};

#[derive(Debug, Clone)]
pub enum FunctionBody {
    Empty,
    Vasm(Vasm),
    RawWasm(Vec<Wat>),
    Import(String, String)
}

impl FunctionBody {
    pub fn is_empty(&self) -> bool {
        match self {
            FunctionBody::Empty => true,
            FunctionBody::Vasm(vasm) => vasm.is_empty(),
            FunctionBody::RawWasm(wat) => wat.is_empty(),
            FunctionBody::Import(_, _) => false,
        }
    }
}