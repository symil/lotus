use parsable::parsable;

#[parsable]
pub struct CommaToken {
    #[parsable(value=",")]
    pub value: String
}