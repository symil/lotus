use std::path::{Path, PathBuf};

use crate::command_line::SRC_DIR_NAME;

pub struct PackageDetails {
    pub root_path: PathBuf,
    pub src_path: PathBuf
}

impl PackageDetails {
    pub fn from_root_path(path: &Path) -> Self {
        let root_path = path.to_path_buf();
        let mut src_path = path.to_path_buf();

        src_path.push(SRC_DIR_NAME);

        Self {
            root_path,
            src_path,
        }
    }
}