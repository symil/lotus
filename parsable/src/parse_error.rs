#[derive(Debug, Clone)]
pub struct ParseError {
    pub file_name: &'static str,
    pub file_namespace: &'static str,
    pub line: usize,
    pub column: usize,
    pub expected: Vec<String>
}