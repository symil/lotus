#[derive(Debug, Clone)]
pub struct ParseError {
    pub line: usize,
    pub col: usize,
    pub expected: Vec<String>
}

impl ParseError {
    pub fn to_string(&self) -> String {
        format!("line {}, column {}: expected {}", self.line, self.col, self.expected.join(" | "))
    }
}