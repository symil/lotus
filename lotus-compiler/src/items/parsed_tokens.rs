use parsable::parsable;

#[parsable]
pub struct ParsedComma {
    #[parsable(value=",")]
    pub token: String
}

#[parsable]
pub struct ParsedSemicolon {
    #[parsable(value=";")]
    pub token: String
}

#[parsable]
pub struct ParsedDot {
    #[parsable(value=".", followed_by="[^.]")] // to avoid working on the `..` operator
    pub token: String,
}

#[parsable]
pub struct ParsedColon {
    #[parsable(value=":")]
    pub token: String
}


#[parsable]
pub struct ParsedDoubleColon {
    #[parsable(value="::")]
    pub token: String
}

#[parsable]
pub struct ParsedArrow {
    #[parsable(value="=>")]
    pub token: String
}

#[parsable]
pub struct ParsedWildcard {
    #[parsable(value="_")]
    pub token: String
}

#[parsable]
pub struct ParsedDoubleDot {
    #[parsable(value="..")]
    pub token: String
}