use std::path::PathBuf;

#[derive(Debug)]
pub struct SourceFileDetails {
    pub path: PathBuf,
    pub root_directory_path: String,
    pub content: String
}