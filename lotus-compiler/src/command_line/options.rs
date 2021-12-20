use std::{path::{PathBuf, Path}, fs};
use crate::{program::{SourceDirectoryDetails, PRELUDE_NAMESPACE, SELF_NAMESPACE}, language_server::LanguageServerAction};
use super::{LogLevel, CARGO_MANIFEST_DIR_PATH, PRELUDE_DIR_NAME, infer_root_directory};

#[derive(Debug)]
pub struct CommandLineOptions {
    pub prelude_path: String,
    pub input_path: String,
    pub output_path: String,
    pub log_level: LogLevel,
    pub language_server_action: Option<LanguageServerAction>
}

impl CommandLineOptions {
    pub fn parse_from_args(args: Vec<String>) -> Option<Self> {
        let mut options = Self {
            prelude_path: get_default_prelude_path(),
            input_path: String::new(),
            output_path: String::new(),
            log_level: LogLevel::Short,
            language_server_action: None
        };

        let mut infer_root = false;

        for arg in &args[1..] {
            match is_option(arg) {
                true => {
                    if let Some(mode) = LanguageServerAction::from_command_line_arg(arg) {
                        options.language_server_action = Some(mode);
                    } else if let Some(log_level) = LogLevel::from_command_line_arg(arg) {
                        options.log_level = log_level;
                    } else if arg == "--infer-root" {
                        infer_root = true;
                    }
                },
                false => {
                    if options.input_path.is_empty() {
                        options.input_path = string_to_absolute_path(&arg).unwrap_or_default();
                    } else if options.output_path.is_empty() {
                        options.output_path = string_to_absolute_path(&arg).unwrap_or_default();
                    }
                },
            }
        }

        if infer_root {
            options.input_path = infer_root_directory(&options.input_path).unwrap_or_default();
        }

        match options.is_valid() {
            true => Some(options),
            false => None,
        }
    }

    fn is_valid(&self) -> bool {
        // TODO: check that the paths are actually valid?
        !self.input_path.is_empty()
    }

    pub fn get_source_directories(&self) -> Vec<SourceDirectoryDetails> {
        let mut result = vec![];

        result.push(SourceDirectoryDetails {
            path: self.prelude_path.clone()
        });

        if self.input_path != self.prelude_path {
            result.push(SourceDirectoryDetails {
                path: self.input_path.clone()
            });
        }

        result
    }
}

fn string_to_absolute_path(string: &str) -> Option<String> {
    let path = Path::new(string);

    match path.canonicalize() {
        Ok(path_buf) => path_buf.as_os_str().to_str().map(|s| s.to_string()),
        Err(_) => None,
    }
}

fn is_option(string: &str) -> bool {
    string.starts_with("-")
}

fn get_default_prelude_path() -> String {
    let mut path_buf = PathBuf::new();

    path_buf.push(CARGO_MANIFEST_DIR_PATH);
    path_buf.push(PRELUDE_DIR_NAME);

    path_buf.into_os_string().into_string().unwrap()
}