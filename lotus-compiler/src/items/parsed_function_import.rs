use parsable::parsable;
use crate::program::{FunctionBody, ProgramContext};
use super::{Identifier, OpeningSquareBracket, ClosingSquareBracket, DotToken};

#[parsable]
pub struct ParsedFunctionImport {
    pub opening_bracket: OpeningSquareBracket,
    #[parsable(value="import")]
    pub import_keyword: String,
    pub first_namespace: Identifier,
    pub dot: DotToken,
    pub second_namespace: Identifier,
    pub closing_bracket: ClosingSquareBracket,
}

impl ParsedFunctionImport {
    pub fn process(&self, context: &mut ProgramContext) -> Option<FunctionBody> {
        Some(FunctionBody::Import(self.first_namespace.to_string(), self.second_namespace.to_string()))
    }
}