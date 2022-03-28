use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct SourceDirectory {
    pub root_path: String,
    pub exclude: Vec<&'static str>
}