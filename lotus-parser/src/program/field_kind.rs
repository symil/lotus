#[derive(Debug, Clone, Copy)]
pub enum FieldKind {
    Regular,
    Static
}

impl FieldKind {
    pub fn get_qualifier(self) -> &'static str {
        match self {
            FieldKind::Regular => "",
            FieldKind::Static => "static ",
        }
    }
}