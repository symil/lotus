use parsable::parsable;

use crate::{generation::Wat, program::{ProgramContext, TypeOld, IrFragment}};

#[parsable(name="float")]
pub struct FloatLiteral {
    #[parsable(regex = r"(-?\d+(\.\d*)?f)|nan")]
    pub value: String,
}

impl FloatLiteral {
    pub fn process(&self, context: &mut ProgramContext) -> Option<IrFragment> {
        let f32_value = match self.value.as_str() {
            "nan" => f32::NAN,
            _ => self.value[0..self.value.len()-1].parse().unwrap()
        };

        Some(IrFragment::simple(context.float_type(), Wat::const_f32(f32_value)))
    }
}