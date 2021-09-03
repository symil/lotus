use parsable::parsable;

use crate::program::ItemVisibility;

#[parsable]
pub struct Visibility {
    pub token: Option<VisibilityToken>
}

#[parsable]
#[derive(PartialEq)]
pub enum VisibilityToken {
    Private = "prv",
    Public = "pub",
    Export = "export",
    System = "sys"
}

impl Visibility {
    pub fn get_token(&self) -> VisibilityToken {
        match &self.token {
            Some(token) => token.clone(),
            None => VisibilityToken::Private,
        }
    }

    pub fn to_item_visibility(&self) -> ItemVisibility {
        match self.get_token() {
            VisibilityToken::Private => ItemVisibility::Private,
            VisibilityToken::Public => ItemVisibility::Public,
            VisibilityToken::Export => ItemVisibility::Export,
            VisibilityToken::System => ItemVisibility::System,
        }
    }
}