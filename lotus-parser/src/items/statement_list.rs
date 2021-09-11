use parsable::parsable;
use crate::program::{ProgramContext, Type, IrFragment};
use super::Statement;

#[parsable]
pub struct StatementList {
    #[parsable(brackets="{}")]
    pub list: Vec<Statement>
}

impl StatementList {
    pub fn process(&self, context: &mut ProgramContext) -> Option<IrFragment> {
        let mut ok = true;
        let mut wat = vec![];
        let mut variables = vec![];

        for statement in &self.list {
            if let Some(wasm) = statement.process(context) {
                wat.extend(wasm.wat);
                variables.extend(wasm.variables);
            } else {
                ok = false;
            }
        }

        match ok {
            true => Some(IrFragment::new(Type::Void, wat, variables)),
            false => None
        }
    }
}