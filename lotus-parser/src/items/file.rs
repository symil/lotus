use parsable::parsable;
use crate::program::ProgramContext;
use super::{FunctionDeclaration, GlobalVarDeclaration, TypeDeclaration, TopLevelBlock};

#[parsable]
pub struct LotusFile {
    pub blocks: Vec<TopLevelBlock>,
}