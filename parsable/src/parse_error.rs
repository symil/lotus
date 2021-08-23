#[derive(Debug, Clone)]
pub struct ParseError {
    pub file_name: &'static str,
    pub namespace_name: &'static str,
    pub line: usize,
    pub column: usize,
    pub expected: Vec<String>
}