#[derive(Debug, Clone, Copy)]
pub enum ItemVisibility {
    Private,
    Public,
    Export,
    System,
}

impl Default for ItemVisibility {
    fn default() -> Self {
        Self::Private
    }
}