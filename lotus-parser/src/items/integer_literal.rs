use parsable::parsable;

use crate::{generation::Wat, program::{ProgramContext, TypeOld, Wasm}};

#[parsable(name="integer")]
pub struct IntegerLiteral {
    #[parsable(regex = r"(-?\d+)|mi")]
    pub value: String,
}

impl IntegerLiteral {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Wasm> {
        let i32_value = match self.value.as_str() {
            "mi" => i32::MIN,
            _ => self.value.parse().unwrap()
        };

        Some(Wasm::simple(TypeOld::Integer, Wat::const_i32(i32_value)))
    }
}