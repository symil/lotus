use enum_iterator::IntoEnumIterator;
use parsable::parsable;

#[parsable]
pub struct ParsedActionKeyword {
    pub token: ParsedActionKeywordToken
}

#[parsable(impl_display=true)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, IntoEnumIterator)]
pub enum ParsedActionKeywordToken {
    Return = "return",
    Check = "check",
    Break = "break",
    Continue = "continue",
    Intercept = "intercept",
    Yield = "yield",
}