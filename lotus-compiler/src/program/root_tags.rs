#[derive(Debug, Clone, Default)]
pub struct RootTags {
    pub disable_main_type_checks: bool,
    pub check_field_access: bool,
}

impl RootTags {
    pub fn new() -> Self {
        Self::default()
    }
}