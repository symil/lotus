#[derive(Debug, Clone)]
pub enum ExpectedKind {
    Expression,
    Identifier,
    Type,
    Keyword,
    Argument,
    FunctionBody,
    Block,
    Token(&'static str),
    TokenAmong(Vec<&'static str>),
    Item(String)
}

impl ExpectedKind {
    pub fn to_string(&self) -> String {
        match self {
            ExpectedKind::Expression => format!("expression"),
            ExpectedKind::Identifier => format!("identifier"),
            ExpectedKind::Type => format!("type"),
            ExpectedKind::Keyword => format!("keyword"),
            ExpectedKind::Argument => format!("argument"),
            ExpectedKind::FunctionBody => format!("function body"),
            ExpectedKind::Block => format!("block"),
            ExpectedKind::Token(token) => format!("\"{}\"", token),
            ExpectedKind::TokenAmong(list) => list.iter().map(|token| format!("\"{}\"", token)).collect::<Vec<String>>().join(" | "),
            ExpectedKind::Item(item) => item.to_string(),
        }
    }
}