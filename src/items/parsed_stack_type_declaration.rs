use parsable::{ItemLocation, parsable};
use crate::program::{STACK_TYPE_KEYWORDS, ProgramContext, WasmStackType, I32_KEYWORD, F32_KEYWORD, VOID_KEYWORD};
use super::{ParsedOpeningRoundBracket, ParsedClosingRoundBracket, unwrap_item};

#[parsable]
pub struct ParsedStackTypeDeclaration {
    pub opening_bracket: ParsedOpeningRoundBracket,
    // pub stack_type: Option<FlexKeywordAmong<STACK_TYPE_KEYWORDS>>,
    pub stack_type: Option<ParsedStackType>,
    pub closing_bracket: Option<ParsedClosingRoundBracket>,
}

#[parsable]
pub enum ParsedStackType {
    I32 = "i32",
    F32 = "f32",
    Void = "void"
}

impl ParsedStackTypeDeclaration {
    pub fn process(&self, context: &mut ProgramContext) -> Option<WasmStackType> {
        let keyword = unwrap_item(&self.stack_type, &self.opening_bracket, context)?;
        let closing_bracket = unwrap_item(&self.closing_bracket, &self.opening_bracket, context)?;

        match keyword {
            ParsedStackType::I32 => Some(WasmStackType::I32),
            ParsedStackType::F32 => Some(WasmStackType::F32),
            ParsedStackType::Void => Some(WasmStackType::Void),
        }
    }
}