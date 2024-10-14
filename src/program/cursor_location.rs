#[derive(Debug, Clone)]
pub struct CursorLocation {
    pub root_directory_path: String,
    pub file_path: String,
    pub index: usize
}

impl CursorLocation {
    pub fn new(root_directory_path: &str, file_path: &str, index: usize) -> Self {
        Self {
            root_directory_path: root_directory_path.to_string(),
            file_path: file_path.to_string(),
            index,
        }
    }
}