use parsable::parsable;
use crate::{program::{ProgramContext, Type, Vasm, Wat}};
use super::ParsedWatExpression;

#[parsable]
pub struct ParsedWatExpressionList {
    #[parsable(prefix="{{", suffix="}}")]
    pub list: Vec<ParsedWatExpression>
}

impl ParsedWatExpressionList {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Vec<Wat>> {
        Some(self.list.iter().map(|item| item.process(context)).collect())
    }
}