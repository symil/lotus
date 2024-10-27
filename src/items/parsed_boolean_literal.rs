use enum_iterator::Sequence;
use parsable::parsable;
use crate::{program::{ProgramContext, Vasm}};

#[parsable(name="boolean")]
pub struct ParsedBooleanLiteral {
    pub token: ParsedBooleanLiteralToken
}

#[parsable]
#[derive(Sequence)]
pub enum ParsedBooleanLiteralToken {
    True = "true",
    False = "false"
}

impl ParsedBooleanLiteral {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Vasm> {
        let i32_value = match &self.token {
            ParsedBooleanLiteralToken::True => 1,
            ParsedBooleanLiteralToken::False => 0,
        };

        let result = context.vasm()
            .int(i32_value)
            .set_type(context.bool_type());

        Some(result)
    }
}