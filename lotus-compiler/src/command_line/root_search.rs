use std::path::Path;
use super::ROOT_FILE_NAME;

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