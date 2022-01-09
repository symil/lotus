use parsable::parsable;

#[parsable]
pub struct ParsedOpeningRoundBracket {
    #[parsable(value="(")]
    pub token: String
}

#[parsable]
pub struct ParsedClosingRoundBracket {
    #[parsable(value=")")]
    pub token: String
}

#[parsable]
pub struct ParsedOpeningCurlyBracket {
    #[parsable(value="{")]
    pub token: String
}

#[parsable]
pub struct ParsedClosingCurlyBracket {
    #[parsable(value="}")]
    pub token: String
}

#[parsable]
pub struct ParsedOpeningSquareBracket {
    #[parsable(value="[")]
    pub token: String
}

#[parsable]
pub struct ParsedClosingSquareBracket {
    #[parsable(value="]")]
    pub token: String
}

#[parsable]
pub struct OpeningAngleBracket {
    #[parsable(value="<")]
    pub token: String
}

#[parsable]
pub struct ParsedClosingAngleBracket {
    #[parsable(value=">")]
    pub token: String
}