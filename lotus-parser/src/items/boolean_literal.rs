use parsable::parsable;

use crate::{generation::Wat, program::{ProgramContext, TypeOld, Wasm}};

#[parsable(name="boolean")]
pub struct BooleanLiteral {
    #[parsable(regex = r"true|false")]
    pub value: String
}

impl BooleanLiteral {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Wasm> {
        let i32_value = match self.value.as_str() {
            "true" => 1,
            "false" => 0,
            _ => {
                dbg!(self.value.as_str());
                unreachable!()
            }
        };

        Some(Wasm::simple(TypeOld::Boolean, Wat::const_i32(i32_value)))
    }
}