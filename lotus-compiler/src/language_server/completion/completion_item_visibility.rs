use super::CompletionItemPosition;

const INTERNAL_ITEM_PREFIX : &'static str = "__";
const PRIVATE_ITEM_PREFIX : &'static str = "_";

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum CompletionItemVisibility {
    Public,
    Private,
    Internal
}

impl CompletionItemVisibility {
    pub fn from_str(item_name: &str) -> Self {
        if item_name.starts_with(INTERNAL_ITEM_PREFIX) {
            Self::Internal
        } else if item_name.starts_with(PRIVATE_ITEM_PREFIX) {
            Self::Private
        } else {
            Self::Public
        }
    }

    pub fn is_internal(&self) -> bool {
        match self {
            CompletionItemVisibility::Public => false,
            CompletionItemVisibility::Private => false,
            CompletionItemVisibility::Internal => true,
        }
    }
}