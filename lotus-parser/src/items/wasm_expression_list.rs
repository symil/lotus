use parsable::parsable;
use crate::{generation::Wat, program::ProgramContext};

use super::WasmExpression;

#[parsable]
pub struct WasmExpressionList {
    #[parsable(prefix="{{", suffix="}}")]
    pub list: Vec<WasmExpression>
}

impl WasmExpressionList {
    pub fn process(&self, context: &mut ProgramContext) -> Vec<Wat> {
        let mut list = vec![];

        for item in &self.list {
            list.push(item.process(context));
        }

        list
    }
}