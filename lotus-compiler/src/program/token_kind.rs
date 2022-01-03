#[derive(Debug, Clone, Copy)]
pub enum TokenKind {
    Expression,
    Identifier,
    Keyword,
    FunctionBody
}

impl TokenKind {
    pub fn to_str(&self) -> &'static str {
        match self {
            TokenKind::Expression => "expression",
            TokenKind::Identifier => "identifier",
            TokenKind::Keyword => "keyword",
            TokenKind::FunctionBody => "function body",
        }
    }
}