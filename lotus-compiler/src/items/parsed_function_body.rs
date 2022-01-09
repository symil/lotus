use parsable::parsable;
use crate::{program::{ProgramContext, Type, Vasm, FunctionBody}};
use super::{BlockExpression, WasmExpressionList};

#[parsable]
pub enum ParsedFunctionBody {
    WebAssembly(WasmExpressionList),
    Block(BlockExpression),
}

impl ParsedFunctionBody {
    pub fn process(&self, type_hint: Option<&Type>, context: &mut ProgramContext) -> Option<FunctionBody> {
        match self {
            ParsedFunctionBody::WebAssembly(wasm) => wasm.process(context).map(|wat| FunctionBody::RawWasm(wat)),
            ParsedFunctionBody::Block(statements) => statements.process(type_hint, context).map(|vasm| FunctionBody::Vasm(vasm)),
        }
    }
    pub fn is_raw_wasm(&self) -> bool {
        match self {
            ParsedFunctionBody::WebAssembly(_) => true,
            ParsedFunctionBody::Block(_) => false,
        }
    }
}