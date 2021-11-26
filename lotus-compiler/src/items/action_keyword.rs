use parsable::parsable;

#[parsable]
pub struct ActionKeywordWrapper {
    pub value: ActionKeyword
}

#[parsable(impl_display=true)]
pub enum ActionKeyword {
    Return = "return",
    Check = "check",
    Break = "break",
    Continue = "continue"
}