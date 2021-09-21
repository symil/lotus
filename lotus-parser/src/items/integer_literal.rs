use parsable::parsable;
use crate::{program::{ProgramContext, VI, Vasm}};

#[parsable(name="integer")]
pub struct IntegerLiteral {
    #[parsable(regex = r"((-|\+)?\d+)|mi")]
    pub value: String,
}

impl IntegerLiteral {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Vasm> {
        let i32_value = match self.value.as_str() {
            "mi" => i32::MIN,
            _ => self.value.parse().unwrap()
        };

        Some(Vasm::new(context.int_type(), vec![], vec![VI::int(i32_value)]))
    }
}