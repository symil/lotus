use parsable::parsable;

#[parsable]
pub struct ParsedSelfKeyword {
    #[parsable(value="self")]
    pub token: String
}

#[parsable]
pub struct ParsedMatchKeyword {
    #[parsable(value="match")]
    pub token: String
}