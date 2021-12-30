use std::path::{Path, PathBuf};
use crate::program::SourceDirectoryDetails;
use super::{ROOT_FILE_NAME, CARGO_MANIFEST_DIR_PATH, PRELUDE_DIR_NAME};

pub fn bundle_with_prelude(input_directory: &str) -> Vec<SourceDirectoryDetails> {
    let mut result = vec![];
    let prelude_path = get_default_prelude_path();

    result.push(SourceDirectoryDetails {
        path: prelude_path.clone()
    });

    if input_directory != &prelude_path {
        result.push(SourceDirectoryDetails {
            path: input_directory.to_string()
        });
    }

    result
}

fn get_default_prelude_path() -> String {
    let mut path_buf = PathBuf::new();

    path_buf.push(CARGO_MANIFEST_DIR_PATH);
    path_buf.push(PRELUDE_DIR_NAME);

    path_buf.into_os_string().into_string().unwrap()
}

pub fn infer_root_directory(file_or_dir_path: &str) -> Option<String> {
    let mut result = None;
    let path = Path::new(file_or_dir_path);

    if path.is_file() {
        if let Some(parent) = path.to_path_buf().parent() {
            if let Some(s) = parent.to_str() {
                result = infer_root_directory(s);
            }
        }
    } else if path.is_dir() {
        if let Ok(entries) = path.read_dir() {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Some(file_name) = entry.path().file_name() {
                        if let Some(file_name_str) = file_name.to_str() {
                            if file_name_str == ROOT_FILE_NAME {
                                result = path.to_str().map(|s| s.to_string());
                                break;
                            }
                        }
                    }
                }
            }

            if result.is_none() {
                if let Some(parent) = path.to_path_buf().parent() {
                    if let Some(s) = parent.to_str() {
                        result = infer_root_directory(s);
                    }
                }
            }
        }
    }

    result
}