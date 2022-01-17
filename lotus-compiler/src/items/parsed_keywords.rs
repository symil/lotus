use parsable::parsable;

#[parsable]
pub struct ParsedSuperKeyword {
    #[parsable(value = "self")]
    pub token: String
}