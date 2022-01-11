use super::CompletionItemVisibility;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum CompletionItemPosition {
    EnumMember,
    PublicVariable,
    PublicFunction,
    PublicType,
    Literal,
    PrivateVariable,
    PrivateFunction,
    InternalVariable,
    InternalFunction,
}

impl CompletionItemPosition {
    pub fn from_visibility(visibility: CompletionItemVisibility, is_function: bool) -> Self {
        match is_function {
            true => match visibility {
                CompletionItemVisibility::Public => CompletionItemPosition::PublicFunction,
                CompletionItemVisibility::Private => CompletionItemPosition::PrivateFunction,
                CompletionItemVisibility::Internal => CompletionItemPosition::InternalFunction,
            },
            false => match visibility {
                CompletionItemVisibility::Public => CompletionItemPosition::PublicVariable,
                CompletionItemVisibility::Private => CompletionItemPosition::PrivateVariable,
                CompletionItemVisibility::Internal => CompletionItemPosition::InternalVariable,
            },
        }
    }
}