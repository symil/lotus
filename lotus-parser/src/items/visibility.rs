use parsable::parsable;

#[parsable]
pub struct VisibilityToken {
    pub value: Option<Visibility>
}

#[parsable]
#[derive(PartialEq)]
pub enum Visibility {
    Private = "prv",
    Public = "pub",
    Export = "export",
    System = "sys",
    Member
}

impl Default for Visibility {
    fn default() -> Self {
        Self::Private
    }
}