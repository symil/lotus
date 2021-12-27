use std::{path::{PathBuf, Path}, fs};
use crate::{program::{SourceDirectoryDetails, PRELUDE_NAMESPACE, SELF_NAMESPACE}, language_server::LanguageServerAction};
use super::{LogLevel, CARGO_MANIFEST_DIR_PATH, PRELUDE_DIR_NAME, infer_root_directory};

#[derive(Debug)]
pub struct CommandLineOptions {
    pub prelude_path: String,
    pub input_path: String,
    pub output_path: String,
    pub log_level: LogLevel,
    pub infer_root: bool,
    pub language_server_action: Option<LanguageServerAction>,
    pub cursor: Option<usize>,
    pub new_name: Option<String>
}

impl CommandLineOptions {
    pub fn parse_from_args(args: Vec<String>) -> Option<Self> {
        let mut options = Self {
            prelude_path: get_default_prelude_path(),
            input_path: String::new(),
            output_path: String::new(),
            log_level: LogLevel::Short,
            infer_root: false,
            language_server_action: None,
            cursor: None,
            new_name: None,
        };

        for arg in &args[1..] {
            match is_option(arg) {
                true => {
                    if let Some(mode) = LanguageServerAction::from_command_line_arg(arg) {
                        options.language_server_action = Some(mode);
                    } else if let Some(log_level) = LogLevel::from_command_line_arg(arg) {
                        options.log_level = log_level;
                    } else if arg == "--infer-root" {
                        options.infer_root = true;
                    } else if let Some(cursor) = get_option_value(arg, "--cursor") {
                        if let Ok(value) = cursor.parse::<usize>() {
                            options.cursor = Some(value);
                        }
                    } else if let Some(new_name) = get_option_value(arg, "--new-name") {
                        options.new_name = Some(new_name);
                    } else {
                        eprintln!("invalid option `{}`", arg);
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

        match options.is_valid() {
            true => Some(options),
            false => None,
        }
    }

    fn is_valid(&self) -> bool {
        // TODO: check that the path is actually valid?
        !self.input_path.is_empty()
    }

    pub fn get_root_directory_path(&self) -> String {
        match self.infer_root {
            true => infer_root_directory(&self.input_path).unwrap_or_default(),
            false => self.input_path.to_string(),
        }
    }

    pub fn get_source_directories(&self) -> Vec<SourceDirectoryDetails> {
        let mut result = vec![];

        result.push(SourceDirectoryDetails {
            path: self.prelude_path.clone()
        });

        let input_path = self.get_root_directory_path();

        if !input_path.is_empty() && input_path != self.prelude_path {
            result.push(SourceDirectoryDetails {
                path: input_path
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

fn get_option_value(arg: &str, option_name: &str) -> Option<String> {
    if arg.starts_with(option_name) && arg.as_bytes().get(option_name.len()).unwrap_or(&0) == &(b'=') {
        return Some(arg[option_name.len() + 1..].to_string());
    }

    None
}