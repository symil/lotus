use parsable::parsable;

use crate::program::Visibility;

#[parsable]
pub struct VisibilityKeyword {
    pub value: VisibilityKeywordValue
}

#[parsable]
#[derive(Clone, Copy, PartialEq)]
pub enum VisibilityKeywordValue {
    Private = "prv",
    Public = "pub",
    Export = "export",
    System = "sys",
}

impl VisibilityKeyword {
    pub fn process_or(item: &Option<Self>, default: Visibility) -> Visibility {
        item.as_ref().map(|item| item.process()).unwrap_or(default)
    }

    pub fn process(&self) -> Visibility {
        match &self.value {
            VisibilityKeywordValue::Private => Visibility::Private,
            VisibilityKeywordValue::Public => Visibility::Public,
            VisibilityKeywordValue::Export => Visibility::Export,
            VisibilityKeywordValue::System => Visibility::System,
        }
    }
}