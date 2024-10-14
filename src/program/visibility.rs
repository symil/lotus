#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Visibility {
    None,
    Private,
    Public,
    Export,
    System,
}

impl Visibility {
    pub fn is_system(&self) -> bool {
        match self {
            Visibility::System => true,
            _ => false
        }
    }
}