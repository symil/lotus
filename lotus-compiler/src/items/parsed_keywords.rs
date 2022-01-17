use parsable::parsable;

#[parsable]
pub struct ParsedSelfKeyword {
    #[parsable(value="self")]
    pub token: String
}