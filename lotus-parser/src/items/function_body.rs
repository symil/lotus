use parsable::parsable;
use crate::{generation::Wat, program::{ProgramContext, Type, Wasm}};
use super::{StatementList, WasmExpressionList};

#[parsable]
pub enum FunctionBody {
    Statements(StatementList),
    WebAssembly(WasmExpressionList)
}

impl FunctionBody {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Wasm> {
        match self {
            FunctionBody::Statements(statements) => statements.process(context),
            FunctionBody::WebAssembly(wasm) => Some(Wasm::new(Type::Void, wasm.process(context), vec![])),
        }
    }
}