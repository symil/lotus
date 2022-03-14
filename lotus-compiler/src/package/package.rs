use std::{path::{Path, PathBuf}, fs};
use toml::Value;
use crate::program::SourceDirectory;
use super::{CONFIG_FILE_NAME, SRC_DIR_NAME, CARGO_MANIFEST_DIR_PATH, PRELUDE_DIR_NAME};

#[derive(Debug, Clone)]
pub struct Package {
    pub root_path: String,
    pub src_path: String,
    pub exclude_framework: bool
}

impl Package {
    pub fn from_path(path: &str) -> Self {
        let root_path = infer_root_directory(Path::new(path)).unwrap();
        let src_path = root_path.join(SRC_DIR_NAME);
        let config_path = root_path.join(CONFIG_FILE_NAME);

        let mut result = Self {
            root_path: root_path.to_string_lossy().to_string(),
            src_path: src_path.to_string_lossy().to_string(),
            exclude_framework: false,
        };

        if let Ok(content) = fs::read_to_string(config_path) {
            if let Ok(config) = content.parse::<Value>() {
                result.exclude_framework = config.get("framework")
                    .and_then(|value| value.as_bool())
                    .map(|b| !b)
                    .unwrap_or(true);
            }
        }

        result
    }

    pub fn get_source_directories(&self) -> Vec<SourceDirectory> {
        let mut result = vec![];
        let prelude_path = get_default_prelude_path();
        let prelude_source_directory = SourceDirectory {
            root_path: prelude_path.join(SRC_DIR_NAME).to_string_lossy().to_string(),
            exclude: match self.exclude_framework {
                true => Some("framework".to_string()),
                false => None,
            },
        };

        result.push(prelude_source_directory);

        if self.root_path != prelude_path.to_str().unwrap() {
            result.push(SourceDirectory {
                root_path: self.src_path.clone(),
                exclude: None,
            });
        }

        result
    }
}

fn get_default_prelude_path() -> PathBuf {
    let mut path_buf = PathBuf::new();

    path_buf.push(CARGO_MANIFEST_DIR_PATH);
    path_buf.push(PRELUDE_DIR_NAME);

    path_buf
}

fn infer_root_directory(path: &Path) -> Option<PathBuf> {
    let mut result = None;

    if path.is_file() {
        if let Some(parent) = path.to_path_buf().parent() {
            result = infer_root_directory(parent);
        }
    } else if path.is_dir() {
        if let Ok(entries) = path.read_dir() {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Some(file_name) = entry.path().file_name() {
                        if let Some(file_name_str) = file_name.to_str() {
                            if file_name_str == CONFIG_FILE_NAME || file_name_str == SRC_DIR_NAME {
                                result = Some(path.to_path_buf());
                                break;
                            }
                        }
                    }
                }
            }

            if result.is_none() {
                if let Some(parent) = path.to_path_buf().parent() {
                    result = infer_root_directory(parent);
                }
            }
        }
    }

    result
}