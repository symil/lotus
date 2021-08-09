use parsable::parsable;

use crate::program::{ProgramContext, Wasm};

use super::Branch;

#[parsable]
pub struct WhileBlock {
    #[parsable(prefix="while")]
    pub while_branch: Branch
}

impl WhileBlock {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Wasm> {
        todo!()
    }
}