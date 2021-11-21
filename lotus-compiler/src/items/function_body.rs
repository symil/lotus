use parsable::parsable;
use crate::{program::{ProgramContext, Type, Vasm}};
use super::{BlockExpression, WasmExpressionList};

#[parsable]
pub enum FunctionBody {
    WebAssembly(WasmExpressionList),
    Block(BlockExpression),
}

impl FunctionBody {
    pub fn process(&self, type_hint: Option<&Type>, context: &mut ProgramContext) -> Option<Vasm> {
        match self {
            FunctionBody::WebAssembly(wasm) => wasm.process(context),
            FunctionBody::Block(statements) => statements.process(type_hint, context),
        }
    }

    pub fn is_raw_wasm(&self) -> bool {
        match self {
            FunctionBody::WebAssembly(_) => true,
            _ => false
        }
    }
}