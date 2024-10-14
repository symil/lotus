#[derive(Debug, Clone, Copy)]
pub enum ItemKind {
    Type,
    Variable,
    Function
}

impl ItemKind {
    pub fn to_str(&self) -> &'static str {
        match self {
            ItemKind::Type => "type",
            ItemKind::Variable => "variable",
            ItemKind::Function => "function",
        }
    }
}