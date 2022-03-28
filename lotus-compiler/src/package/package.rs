use std::{path::{Path, PathBuf}, fs};
use toml::Value;
use crate::program::SourceDirectory;
use super::{CONFIG_FILE_NAME, SRC_DIR_NAME, CARGO_MANIFEST_DIR_PATH, PRELUDE_DIR_NAME, CACHE_DIR_NAME, DATA_DIR_NAME};

#[derive(Debug, Clone)]
pub struct Package {
    pub root_path: PathBuf,
    pub src_path: PathBuf,
    pub cache_path: PathBuf,
    pub data_path: PathBuf,
    pub exclude_framework: bool,
    pub exclude_engine: bool,
    pub no_alloc: bool,
}

impl Package {
    pub fn from_path(path: &str) -> Self {
        let root_path = infer_root_directory(Path::new(path)).unwrap();
        let src_path = root_path.join(SRC_DIR_NAME);
        let config_path = root_path.join(CONFIG_FILE_NAME);
        let cache_path = root_path.join(CACHE_DIR_NAME);
        let data_path = cache_path.join(DATA_DIR_NAME);

        let mut result = Self {
            root_path,
            src_path,
            cache_path,
            data_path,
            exclude_framework: true,
            exclude_engine: false,
            no_alloc: false,
        };

        if let Ok(content) = fs::read_to_string(config_path) {
            if let Ok(config) = content.parse::<Value>() {
                result.exclude_framework = config.get("framework")
                    .and_then(|value| value.as_bool())
                    .map(|b| !b)
                    .unwrap_or(true);
                
                result.exclude_engine = config.get("engine")
                    .and_then(|value| value.as_bool())
                    .map(|b| !b)
                    .unwrap_or(false);
                
                result.no_alloc = config.get("no-alloc")
                    .and_then(|value| value.as_bool())
                    .unwrap_or(false);
            }
        }

        if result.exclude_engine {
            result.exclude_framework = true;
        }

        if result.no_alloc {
            result.exclude_engine = true;
            result.exclude_framework = true;
        }

        result
    }

    pub fn get_source_directories(&self) -> Vec<SourceDirectory> {
        let mut result = vec![];
        let prelude_path = get_default_prelude_path();
        let mut exclude = vec![];

        if self.exclude_engine {
            exclude.push("engine");
        }

        if self.exclude_framework {
            exclude.push("framework");
        }

        let prelude_source_directory = SourceDirectory {
            root_path: prelude_path.join(SRC_DIR_NAME).to_string_lossy().to_string(),
            exclude,
        };

        result.push(prelude_source_directory);

        if self.root_path != prelude_path {
            result.push(SourceDirectory {
                root_path: self.src_path.to_string_lossy().to_string(),
                exclude: vec![],
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