use parsable::{create_token_struct, parsable};

#[parsable]
pub struct ParsedDotToken {
    #[parsable(value=".", followed_by="[^.]")] // to avoid the confusion with the range operator `..`
    pub token: String,
}

create_token_struct!(ParsedCommaToken, ",");
create_token_struct!(ParsedSemicolonToken, ";");
create_token_struct!(ParsedColonToken, ":");
create_token_struct!(ParsedDoubleColonToken, "::");
create_token_struct!(ParsedArrowToken, "=>");
create_token_struct!(ParsedWildcardToken, "_");
create_token_struct!(ParsedDoubleDotToken, "..");
create_token_struct!(ParsedEqualToken, "=");
create_token_struct!(ParsedHashToken, "#");
create_token_struct!(ParsedAtToken, "@");

create_token_struct!(ParsedOpeningRoundBracket, "(");
create_token_struct!(ParsedClosingRoundBracket, ")");
create_token_struct!(ParsedOpeningCurlyBracket, "{");
create_token_struct!(ParsedClosingCurlyBracket, "}");
create_token_struct!(ParsedOpeningSquareBracket, "[");
create_token_struct!(ParsedClosingSquareBracket, "]");
create_token_struct!(ParsedOpeningAngleBracket, "<");
create_token_struct!(ParsedClosingAngleBracket, ">");