#[derive(Debug, Default, Clone)]
pub struct DataLocation {
    pub start: usize,
    pub end: usize,
    pub file_name: &'static str,
    pub line: usize,
    pub column: usize
}