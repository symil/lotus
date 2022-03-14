use parsable::{ItemLocation, parsable, Token};
use crate::program::{ProgramContext, Vasm, Type};
use super::{ParsedOpeningRoundBracket, ParsedClosingRoundBracket, ParsedStringLiteral};

#[parsable(cascade = true)]
pub struct ParsedLoadDirective {
    pub token: Token<"#LOAD">,
    pub opening_bracket: Option<ParsedOpeningRoundBracket>,
    pub sheet_name: Option<ParsedStringLiteral>,
    #[parsable(cascade = false)]
    pub closing_bracket: Option<ParsedClosingRoundBracket>,
}

impl ParsedLoadDirective {
    pub fn process(&self, type_hint: Option<&Type>, context: &mut ProgramContext) -> Option<Vasm> {
        None
    }
}