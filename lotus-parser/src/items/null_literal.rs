use parsable::parsable;

use crate::{generation::{NULL_ADDR, Wat}, program::{ProgramContext, Type, Wasm}};

#[parsable(name="null")]
pub struct NullLiteral {
    #[parsable(regex = r"null")]
    value: String
}

impl NullLiteral {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Wasm> {
        Some(Wasm::typed(Type::Null, Wat::const_i32(NULL_ADDR)))
    }
}