#[derive(Debug, Clone)]
pub struct ParseError {
    pub line: usize,
    pub column: usize,
    pub expected: Vec<String>
}

impl ParseError {
    pub fn to_string(&self) -> String {
        format!("line {}, column {}: expected {}", self.line, self.column, self.expected.join(" | "))
    }
}