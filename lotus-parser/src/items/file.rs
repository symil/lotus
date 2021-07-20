use parsable::parsable;

use super::{top_level_block::TopLevelBlock};

#[parsable(located)]
pub struct LotusFile {
    pub blocks: Vec<TopLevelBlock>
}