use parsable::parsable;

#[parsable]
pub struct CommaToken {
    #[parsable(value=",")]
    pub value: String
}

#[parsable]
pub struct SemicolonToken {
    #[parsable(value=";")]
    pub value: String
}

#[parsable]
pub struct DotToken {
    #[parsable(value=".", followed_by="[^.]")] // to avoid working on the `..` operator
    pub dot: String,
}