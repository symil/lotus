use std::{path::{PathBuf, Path}, fs};
use crate::{program::{SourceDirectoryDetails, PRELUDE_NAMESPACE, SELF_NAMESPACE}, language_server::LanguageServerCommand};
use super::{LogLevel, CARGO_MANIFEST_DIR_PATH, PRELUDE_DIR_NAME, infer_root_directory};

#[derive(Debug)]
pub struct CommandLineOptions {
    pub input_path: Option<String>,
    pub output_path: Option<String>,
    pub log_level: LogLevel,
    pub run_as_server: bool
}

impl CommandLineOptions {
    pub fn parse_from_args(args: Vec<String>) -> Self {
        let mut options = Self {
            input_path: None,
            output_path: None,
            log_level: LogLevel::Short,
            run_as_server: false
        };

        for arg in &args[1..] {
            match is_option(arg) {
                true => {
                    if let Some(log_level) = LogLevel::from_command_line_arg(arg) {
                        options.log_level = log_level;
                    } else if arg == "--server" {
                        options.run_as_server = true;
                    } else {
                        eprintln!("invalid option `{}`", arg);
                    }
                },
                false => {
                    if options.input_path.is_none() {
                        options.input_path = Some(string_to_absolute_path(&arg).unwrap_or_default());
                    } else if options.output_path.is_none() {
                        options.output_path = Some(string_to_absolute_path(&arg).unwrap_or_default());
                    }
                },
            }
        }

        options
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