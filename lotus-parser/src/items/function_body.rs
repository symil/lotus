use parsable::parsable;
use crate::{generation::Wat, program::{ProgramContext, Type, IrFragment}};
use super::{StatementList, WasmExpressionList};

#[parsable]
pub enum FunctionBody {
    WebAssembly(WasmExpressionList),
    Statements(StatementList),
}

impl FunctionBody {
    pub fn process(&self, context: &mut ProgramContext) -> Option<IrFragment> {
        match self {
            FunctionBody::WebAssembly(wasm) => Some(IrFragment::new(Type::Void, wasm.process(context), vec![])),
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