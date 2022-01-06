#[derive(Debug, Clone)]
pub struct CursorInfo {
    pub file_path: String,
    pub index: usize
}

impl CursorInfo {
    pub fn new(file_path: &str, index: usize) -> Self {
        Self {
            file_path: file_path.to_string(),
            index,
        }
    }
}