use parsable::parsable;

use crate::program::{ProgramContext, Wasm};

use super::{Expression, Identifier, Statement};

#[parsable]
pub struct ForBlock {
    #[parsable(prefix="for")]
    pub iterator: ForIterator,
    #[parsable(prefix="in")]
    pub array_expression: Expression,
    #[parsable(brackets="{}")]
    pub statements: Vec<Statement>
}

#[parsable]
pub struct IndexAndItem {
    #[parsable(prefix="(")]
    pub index_name: Identifier,
    #[parsable(prefix=",", suffix=")")]
    pub item_name: Identifier
}

#[parsable]
pub enum ForIterator {
    Item(Identifier),
    IndexAndItem(IndexAndItem)
}

impl ForBlock {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Wasm> {
        todo!()
    }
}