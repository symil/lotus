#[derive(Debug, Clone, Copy)]
pub enum ExpectedItemKind {
    Expression,
    Identifier,
    Keyword,
    FunctionBody,
    Token(&'static str)
}

impl ExpectedItemKind {
    pub fn to_string(&self) -> String {
        match self {
            ExpectedItemKind::Expression => format!("expression"),
            ExpectedItemKind::Identifier => format!("identifier"),
            ExpectedItemKind::Keyword => format!("keyword"),
            ExpectedItemKind::FunctionBody => format!("function body"),
            ExpectedItemKind::Token(token) => format!("token `{}`", token),
        }
    }
}