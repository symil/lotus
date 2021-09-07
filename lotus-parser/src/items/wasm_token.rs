use parsable::parsable;

use crate::{generation::Wat, program::ProgramContext};

#[parsable]
pub struct WasmToken {
    #[parsable(regex=r"\w+(\.\w+)?")]
    pub value: String
}

impl WasmToken {
    pub fn process(&self, context: &mut ProgramContext) -> Wat {
        Wat::single(&self.value)
    }
}