use parsable::parsable;
use crate::{generation::Wat, program::{ProgramContext, Type, Vasm}};
use super::{StatementList, WasmExpressionList};

#[parsable]
pub enum FunctionBody {
    WebAssembly(WasmExpressionList),
    Statements(StatementList),
}

impl FunctionBody {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Vasm> {
        match self {
            FunctionBody::WebAssembly(wasm) => wasm.process(context),
            FunctionBody::Statements(statements) => statements.process(context),
        }
    }

    pub fn is_raw_wasm(&self) -> bool {
        match self {
            FunctionBody::WebAssembly(_) => true,
            _ => false
        }
    }
}