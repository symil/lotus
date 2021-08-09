use parsable::parsable;

use crate::program::{ProgramContext, Wasm};

use super::Branch;

#[parsable]
pub struct IfBlock {
    #[parsable(prefix="if")]
    pub if_branch: Branch,
    #[parsable(prefix="else if", separator="else if", optional=true)]
    pub else_if_branches: Vec<Branch>,
    #[parsable(prefix="else")]
    pub else_branch: Option<Branch>
}

impl IfBlock {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Wasm> {
        todo!()
    }
}