use parsable::parsable;

#[parsable]
pub struct ActionKeyword {
    pub token: ActionKeywordToken
}

#[parsable(impl_display=true)]
pub enum ActionKeywordToken {
    Return = "return",
    Break = "break",
    Continue = "continue"
}