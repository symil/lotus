use parsable::parsable;
use crate::program::Visibility;

#[parsable]
pub struct ParsedVisibility {
    pub token: ParsedVisibilityToken
}

#[parsable]
#[derive(Clone, Copy, PartialEq)]
pub enum ParsedVisibilityToken {
    Public = "pub",
    Export = "export",
    System = "sys",
}

impl ParsedVisibility {
    pub fn process_or(item: &Option<Self>, default: Visibility) -> Visibility {
        item.as_ref().map(|item| item.process()).unwrap_or(default)
    }

    pub fn process(&self) -> Visibility {
        match &self.token {
            ParsedVisibilityToken::Public => Visibility::Public,
            ParsedVisibilityToken::Export => Visibility::Export,
            ParsedVisibilityToken::System => Visibility::System,
        }
    }
}