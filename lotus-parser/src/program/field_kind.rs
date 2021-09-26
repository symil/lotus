#[derive(Debug, Clone, Copy, PartialEq)]
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

    pub fn is_static(self) -> bool {
        self == FieldKind::Static
    }
}