use parsable::parsable;

#[parsable]
pub struct Visibility {
    pub token: Option<VisibilityToken>
}

#[parsable]
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
}