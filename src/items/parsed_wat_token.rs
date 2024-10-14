use parsable::parsable;
use crate::{program::{ProgramContext, Wat}};

#[parsable]
pub struct ParsedWatToken {
    #[parsable(regex=r"-?\$?[<>\w:.]+")]
    pub token: String
}

impl ParsedWatToken {
    pub fn process(&self, context: &mut ProgramContext) -> Wat {
        Wat::single(&self.token)
    }
}