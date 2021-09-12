use parsable::parsable;
use crate::{generation::Wat, program::{ProgramContext, Type, VI, Vasm}};

use super::WasmExpression;

#[parsable]
pub struct WasmExpressionList {
    #[parsable(prefix="{{", suffix="}}")]
    pub list: Vec<WasmExpression>
}

impl WasmExpressionList {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Vasm> {
        let mut content = vec![];

        for item in &self.list {
            content.push(VI::Raw(item.process(context)));
        }

        Some(Vasm::new(Type::Void, vec![], content))
    }
}