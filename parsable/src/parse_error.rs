#[derive(Debug, Clone)]
pub struct ParseError {
    pub package_root_path: &'static str,
    pub file_path: &'static str,
    pub file_content: &'static str,
    pub index: usize,
    pub expected: Vec<String>
}