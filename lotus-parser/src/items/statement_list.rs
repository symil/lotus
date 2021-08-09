use parsable::parsable;
use crate::program::{ProgramContext, Wasm};
use super::Statement;

#[parsable]
pub struct StatementList {
    pub list: Vec<Statement>
}

impl StatementList {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Wasm> {
        let mut ok = true;
        let mut wat = vec![];

        for statement in &self.list {
            if let Some(wasm) = statement.process(context) {
                wat.extend(wasm.wat);
            } else {
                ok = false;
            }
        }

        match ok {
            true => Some(Wasm::untyped(wat)),
            false => None
        }
    }
}