use parsable::parsable;
use crate::{program::{ProgramContext, VI, Vasm}};

#[parsable(name="boolean")]
pub struct BooleanLiteral {
    #[parsable(regex = r"true|false")]
    pub value: String
}

impl BooleanLiteral {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Vasm> {
        let i32_value = match self.value.as_str() {
            "true" => 1,
            "false" => 0,
            _ => unreachable!()
        };

        let result = context.vasm()
            .set_type(context.bool_type())
            .int(i32_value);

        Some(result)
    }
}