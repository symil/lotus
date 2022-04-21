#[derive(Debug, Clone, Default)]
pub struct RootTags {
    pub check_main_types: bool,
    pub check_field_access: bool,
}

impl RootTags {
    pub fn new() -> Self {
        Self::default()
    }
}