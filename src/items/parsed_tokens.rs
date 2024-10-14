use parsable::{parsable, Token};
use super::FlexKeyword;

#[parsable]
pub struct ParsedDotToken {
    #[parsable(value=".", followed_by="[^.]")] // to avoid the confusion with the range operator `..`
    pub token: String,
}

pub type ParsedCommaToken = Token<",">;
pub type ParsedSemicolonToken = Token<";">;
pub type ParsedColonToken = Token<":">;
pub type ParsedDoubleColonToken = Token<"::">;
pub type ParsedArrowToken = Token<"=>">;
pub type ParsedWildcardToken = Token<"_">;
pub type ParsedDoubleDotToken = Token<"..">;
pub type ParsedEqualToken = Token<"=">;
pub type ParsedHashToken = Token<"#">;
pub type ParsedAtToken = Token<"@">;

pub type ParsedOpeningRoundBracket = Token<"(">;
pub type ParsedClosingRoundBracket = Token<")">;
pub type ParsedOpeningCurlyBracket = Token<"{">;
pub type ParsedClosingCurlyBracket = Token<"}">;
pub type ParsedOpeningSquareBracket = Token<"[">;
pub type ParsedClosingSquareBracket = Token<"]">;
pub type ParsedOpeningAngleBracket = Token<"<">;
pub type ParsedClosingAngleBracket = Token<">">;