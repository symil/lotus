use parsable::parsable;

use crate::{generation::Wat, program::{ProgramContext, TypeOld, IrFragment}};

#[parsable(name="integer")]
pub struct IntegerLiteral {
    #[parsable(regex = r"(-?\d+)|mi")]
    pub value: String,
}

impl IntegerLiteral {
    pub fn process(&self, context: &mut ProgramContext) -> Option<IrFragment> {
        let i32_value = match self.value.as_str() {
            "mi" => i32::MIN,
            _ => self.value.parse().unwrap()
        };

        Some(IrFragment::simple(context.int_type(), Wat::const_i32(i32_value)))
    }
}