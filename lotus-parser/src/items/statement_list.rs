use parsable::parsable;
use crate::program::{ProgramContext, Type, Vasm};
use super::Statement;

#[parsable]
pub struct StatementList {
    #[parsable(brackets="{}")]
    pub list: Vec<Statement>
}

impl StatementList {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Vasm> {
        let mut source = vec![];

        for statement in &self.list {
            if let Some(vasm) = statement.process(context) {
                source.push(vasm);
            }
        }

        Some(Vasm::merge(source))
    }
}