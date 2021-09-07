use parsable::parsable;
use crate::{generation::Wat, program::ProgramContext};

use super::WasmToken;

#[parsable]
pub enum WasmExpression {
    Leaf(WasmToken),
    #[parsable(brackets="()")]
    Tree(WasmToken, Vec<WasmExpression>)
}

impl WasmExpression {
    pub fn process(&self, context: &mut ProgramContext) -> Wat {
        match self {
            WasmExpression::Leaf(token) => token.process(context),
            WasmExpression::Tree(keyword, items) => {
                let mut wat = keyword.process(context);

                for wasm_expression in items {
                    wat.push(wasm_expression.process(context));
                }
            
                wat
            },
        }
    }
}