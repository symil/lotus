use enum_iterator::Sequence;
use parsable::parsable;

#[parsable]
pub struct ParsedActionKeyword {
    pub token: ParsedActionKeywordToken
}

#[parsable(impl_display=true)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Sequence)]
pub enum ParsedActionKeywordToken {
    Return = "return",
    Check = "check",
    Break = "break",
    Continue = "continue",
    Intercept = "intercept",
    Yield = "yield",
}