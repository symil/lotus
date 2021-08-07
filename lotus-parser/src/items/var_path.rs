use parsable::parsable;

use crate::program::{ProgramContext, Wasm};

use super::{VarPathRoot, VarPathSegment};

#[parsable]
pub struct VarPath {
    pub root: VarPathRoot,
    pub path: Vec<VarPathSegment>
}

impl VarPath {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Wasm> {
        todo!()
    }
}