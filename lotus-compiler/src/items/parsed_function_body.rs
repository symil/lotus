use parsable::parsable;
use crate::{program::{ProgramContext, Type, Vasm, FunctionBody}};
use super::{BlockExpression, WasmExpressionList, ParsedFunctionImport};

#[parsable]
pub enum ParsedFunctionBody {
    WebAssembly(WasmExpressionList),
    Block(BlockExpression),
    Import(ParsedFunctionImport)
}

impl ParsedFunctionBody {
    pub fn process(&self, type_hint: Option<&Type>, context: &mut ProgramContext) -> Option<FunctionBody> {
        match self {
            ParsedFunctionBody::WebAssembly(wasm) => wasm.process(context).map(|wat| FunctionBody::RawWasm(wat)),
            ParsedFunctionBody::Block(statements) => statements.process(type_hint, context).map(|vasm| FunctionBody::Vasm(vasm)),
            ParsedFunctionBody::Import(import) => import.process(context),
        }
    }
    pub fn is_raw_wasm(&self) -> bool {
        match self {
            ParsedFunctionBody::WebAssembly(_) => true,
            ParsedFunctionBody::Block(_) => false,
            ParsedFunctionBody::Import(_) => true,
        }
    }
}