use std::path::{Path, PathBuf};
use crate::program::PackageDetails;
use super::{ROOT_FILE_NAME, CARGO_MANIFEST_DIR_PATH, PRELUDE_DIR_NAME, SRC_DIR_NAME};

pub fn bundle_with_prelude(input_directory: &str) -> Vec<PackageDetails> {
    let mut result = vec![];
    let prelude_path = get_default_prelude_path();

    result.push(PackageDetails::from_root_path(Path::new(&prelude_path)));

    if input_directory != &prelude_path {
        result.push(PackageDetails::from_root_path(Path::new(&input_directory)));
    }

    result
}

fn get_default_prelude_path() -> String {
    let mut path_buf = PathBuf::new();

    path_buf.push(CARGO_MANIFEST_DIR_PATH);
    path_buf.push(PRELUDE_DIR_NAME);

    path_buf.into_os_string().into_string().unwrap()
}

pub fn infer_root_directory(path: &str) -> Option<String> {
    infer_root_directory_inner(Path::new(path)).map(|path_buf| path_buf.as_os_str().to_str().unwrap().to_string())
}

fn infer_root_directory_inner(path: &Path) -> Option<PathBuf> {
    let mut result = None;

    if path.is_file() {
        if let Some(parent) = path.to_path_buf().parent() {
            result = infer_root_directory_inner(parent);
        }
    } else if path.is_dir() {
        if let Ok(entries) = path.read_dir() {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Some(file_name) = entry.path().file_name() {
                        if let Some(file_name_str) = file_name.to_str() {
                            if file_name_str == ROOT_FILE_NAME || file_name_str == SRC_DIR_NAME {
                                result = Some(path.to_path_buf());
                                break;
                            }
                        }
                    }
                }
            }

            if result.is_none() {
                if let Some(parent) = path.to_path_buf().parent() {
                    result = infer_root_directory_inner(parent);
                }
            }
        }
    }

    result
}