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

    pub fn from_is_static(is_static: bool) -> Self {
        match is_static {
            true => FieldKind::Static,
            false => FieldKind::Regular,
        }
    }
}