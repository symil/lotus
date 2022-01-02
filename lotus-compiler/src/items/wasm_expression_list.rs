use parsable::parsable;
use crate::{program::{ProgramContext, Type, VI, Vasm}};

use super::WasmExpression;

#[parsable]
pub struct WasmExpressionList {
    #[parsable(prefix="{{", suffix="}}")]
    pub list: Vec<WasmExpression>
}

impl WasmExpressionList {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Vasm> {
        let mut result = context.vasm();

        for item in &self.list {
            result = result.raw(item.process(context));
        }

        Some(result)
    }
}