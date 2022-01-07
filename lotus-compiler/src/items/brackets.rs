use parsable::parsable;

#[parsable]
pub struct OpeningRoundBracket {
    #[parsable(value="(")]
    pub value: String
}

#[parsable]
pub struct ClosingRoundBracket {
    #[parsable(value=")")]
    pub value: String
}

#[parsable]
pub struct OpeningCurlyBracket {
    #[parsable(value="{")]
    pub value: String
}

#[parsable]
pub struct ClosingCurlyBracket {
    #[parsable(value="}")]
    pub value: String
}

#[parsable]
pub struct OpeningSquareBracket {
    #[parsable(value="[")]
    pub value: String
}

#[parsable]
pub struct ClosingSquareBracket {
    #[parsable(value="]")]
    pub value: String
}

#[parsable]
pub struct OpeningAngleBracket {
    #[parsable(value="<")]
    pub value: String
}

#[parsable]
pub struct ClosingAngleBracket {
    #[parsable(value=">")]
    pub value: String
}