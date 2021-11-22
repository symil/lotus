use parsable::parsable;

#[parsable]
pub struct VisibilityWrapper {
    pub value: Option<Visibility>
}

#[parsable]
#[derive(Clone, Copy, PartialEq)]
pub enum Visibility {
    Private = "prv",
    Public = "pub",
    Export = "export",
    System = "sys",
    None
}

impl Default for Visibility {
    fn default() -> Self {
        Self::Private
    }
}