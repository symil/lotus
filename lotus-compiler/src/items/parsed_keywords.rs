use parsable::parsable;

#[parsable]
pub struct ParsedSuperKeyword {
    #[parsable(value = "super")]
    pub token: String
}