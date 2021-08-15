use parsable::parsable;
use crate::program::{ProgramContext, Wasm};
use super::Statement;

#[parsable]
pub struct StatementList {
    #[parsable(brackets="{}")]
    pub list: Vec<Statement>
}

impl StatementList {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Wasm> {
        let mut ok = true;
        let mut wat = vec![];
        let mut variables = vec![];

        for statement in &self.list {
            if let Some(wasm) = statement.process(context) {
                wat.extend(wasm.wat);
                variables.extend(wasm.declared_variables);
            } else {
                ok = false;
            }
        }

        match ok {
            true => Some(Wasm::untyped(wat, variables)),
            false => None
        }
    }
}