use parsable::parsable;

use crate::{generation::Wat, program::{ProgramContext, VI, Vasm}};

#[parsable(name="float")]
pub struct FloatLiteral {
    #[parsable(regex = r"(-?\d+(\.\d*)?f)|nan")]
    pub value: String,
}

impl FloatLiteral {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Vasm> {
        let f32_value = match self.value.as_str() {
            "nan" => f32::NAN,
            _ => self.value[0..self.value.len() - 1].parse().unwrap()
        };

        Some(Vasm::new(context.float_type(), vec![], vec![VI::float(f32_value)]))
    }
}