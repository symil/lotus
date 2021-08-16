use parsable::parsable;
use crate::program::ProgramContext;
use super::{TopLevelBlock};

#[parsable]
pub struct LotusFile {
    pub blocks: Vec<TopLevelBlock>,
    #[parsable(ignore)]
    pub file_name: String,
    #[parsable(ignore)]
    pub namespace_name: String
}