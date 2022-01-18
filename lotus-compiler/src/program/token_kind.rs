#[derive(Debug, Clone, Copy)]
pub enum ExpectedKind {
    Expression,
    Identifier,
    Keyword,
    Argument,
    FunctionBody,
    Block,
    Token(&'static str),
    Item(&'static str)
}

impl ExpectedKind {
    pub fn to_string(&self) -> String {
        match self {
            ExpectedKind::Expression => format!("expression"),
            ExpectedKind::Identifier => format!("identifier"),
            ExpectedKind::Keyword => format!("keyword"),
            ExpectedKind::Argument => format!("argument"),
            ExpectedKind::FunctionBody => format!("function body"),
            ExpectedKind::Block => format!("block"),
            ExpectedKind::Token(token) => format!("`{}`", token),
            ExpectedKind::Item(item) => item.to_string(),
        }
    }
}