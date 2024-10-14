use parsable::parsable;
use crate::program::{FunctionBody, ProgramContext};
use super::{Identifier, ParsedOpeningSquareBracket, ParsedClosingSquareBracket, ParsedDotToken};

#[parsable]
pub struct ParsedFunctionImport {
    pub opening_bracket: ParsedOpeningSquareBracket,
    #[parsable(value="import")]
    pub import_keyword: String,
    pub first_namespace: Identifier,
    pub dot: ParsedDotToken,
    pub second_namespace: Identifier,
    pub closing_bracket: ParsedClosingSquareBracket,
}

impl ParsedFunctionImport {
    pub fn process(&self, context: &mut ProgramContext) -> Option<FunctionBody> {
        Some(FunctionBody::Import(self.first_namespace.to_string(), self.second_namespace.to_string()))
    }
}