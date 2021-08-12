use parsable::parsable;
use crate::program::ProgramContext;
use super::{TopLevelBlock};

#[parsable]
pub struct LotusFile {
    pub blocks: Vec<TopLevelBlock>
}