use parsable::parsable;
use crate::{program::{ProgramContext, Vasm}};

#[parsable(name="boolean")]
pub struct ParsedBooleanLiteral {
    #[parsable(regex = r"true|false")]
    pub token: String
}

impl ParsedBooleanLiteral {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Vasm> {
        let i32_value = match self.token.as_str() {
            "true" => 1,
            "false" => 0,
            _ => unreachable!()
        };

        let result = context.vasm()
            .int(i32_value)
            .set_type(context.bool_type());

        Some(result)
    }
}