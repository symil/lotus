use parsable::parsable;

use super::{top_level_block::TopLevelBlock};

#[parsable]
pub struct LotusFile {
    pub blocks: Vec<TopLevelBlock>
}