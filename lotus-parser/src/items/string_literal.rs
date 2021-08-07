use parsable::parsable;

use crate::{generation::Wat, program::{ProgramContext, Type, Wasm}};

#[parsable(name="string")]
pub struct StringLiteral {
    #[parsable(regex = r##""(\\.|[^"\\])*""##)]
    pub value: String
}

impl StringLiteral {
    pub fn to_actual_string(&self) -> String {
        // TODO: remove quotes and unescape the string
        self.value.clone()
    }

    pub fn process(&self, context: &mut ProgramContext) -> Option<Wasm> {
        todo!()
    }
}