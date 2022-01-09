use parsable::parsable;
use crate::{program::{ProgramContext, Type, Vasm, Wat}};

use super::WasmExpression;

#[parsable]
pub struct WasmExpressionList {
    #[parsable(prefix="{{", suffix="}}")]
    pub list: Vec<WasmExpression>
}

impl WasmExpressionList {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Vec<Wat>> {
        Some(self.list.iter().map(|item| item.process(context)).collect())
    }
}