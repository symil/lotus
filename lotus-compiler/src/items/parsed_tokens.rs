use parsable::parsable;

#[parsable]
pub struct ParsedCommaToken {
    #[parsable(value=",")]
    pub token: String
}

#[parsable]
pub struct ParsedSemicolonToken {
    #[parsable(value=";")]
    pub token: String
}

#[parsable]
pub struct ParsedDotToken {
    #[parsable(value=".", followed_by="[^.]")] // to avoid working on the `..` operator
    pub token: String,
}

#[parsable]
pub struct ParsedColonToken {
    #[parsable(value=":")]
    pub token: String
}

#[parsable]
pub struct ParsedDoubleColonToken {
    #[parsable(value="::")]
    pub token: String
}

#[parsable]
pub struct ParsedArrowToken {
    #[parsable(value="=>")]
    pub token: String
}

#[parsable]
pub struct ParsedWildcardToken {
    #[parsable(value="_")]
    pub token: String
}

#[parsable]
pub struct ParsedDoubleDotToken {
    #[parsable(value="..")]
    pub token: String
}

#[parsable]
pub struct ParsedEqualToken {
    #[parsable(value="=")]
    pub token: String
}