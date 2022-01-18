use parsable::{ItemLocation, parsable};
use crate::program::{STACK_TYPE_KEYWORDS, ProgramContext, WasmStackType, I32_KEYWORD, F32_KEYWORD, VOID_KEYWORD};
use super::{ParsedOpeningRoundBracket, FlexKeywordAmong, ParsedClosingRoundBracket, unwrap_item};

#[parsable]
pub struct ParsedStackTypeDeclaration {
    pub opening_bracket: ParsedOpeningRoundBracket,
    pub stack_type: Option<FlexKeywordAmong<STACK_TYPE_KEYWORDS>>,
    pub closing_bracket: Option<ParsedClosingRoundBracket>,
}

impl ParsedStackTypeDeclaration {
    pub fn process(&self, context: &mut ProgramContext) -> Option<WasmStackType> {
        let keyword = unwrap_item(&self.stack_type, &self.opening_bracket, context)?;
        let closing_bracket = unwrap_item(&self.closing_bracket, keyword, context)?;

        match keyword.process(context) {
            Some(I32_KEYWORD) => Some(WasmStackType::I32),
            Some(F32_KEYWORD) => Some(WasmStackType::F32),
            Some(VOID_KEYWORD) => Some(WasmStackType::Void),
            _ => None,
        }
    }
}