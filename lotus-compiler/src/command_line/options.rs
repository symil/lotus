use std::{path::{PathBuf, Path}};
use crate::{program::{PackageDetails, PRELUDE_NAMESPACE, SELF_NAMESPACE}, language_server::LanguageServerCommandKind};
use super::{LogLevel, CARGO_MANIFEST_DIR_PATH, PRELUDE_DIR_NAME, infer_root_directory};

#[derive(Debug)]
pub struct CommandLineOptions {
    pub input_path: Option<String>,
    pub output_path: Option<String>,
    pub log_level: LogLevel,
    pub run_as_server: bool,
    pub run_benchmark: bool,
    pub command: Option<String>,
}

impl CommandLineOptions {
    pub fn parse_from_args(args: Vec<String>) -> Self {
        let mut options = Self {
            input_path: None,
            output_path: None,
            log_level: LogLevel::Short,
            run_as_server: false,
            run_benchmark: false,
            command: None,
        };

        for arg in &args[1..] {
            match is_option(arg) {
                true => {
                    if let Some(log_level) = LogLevel::from_command_line_arg(arg) {
                        options.log_level = log_level;
                    } else if arg == "--benchmark" {
                        options.run_benchmark = true;
                    } else if arg == "--server" {
                        options.run_as_server = true;
                    } else if let Some(command) = get_option_value(arg, "--command") {
                        options.command = Some(command);
                    } else {
                        eprintln!("invalid option `{}`", arg);
                    }
                },
                false => {
                    if options.input_path.is_none() {
                        options.input_path = string_to_absolute_path(&arg);
                    } else if options.output_path.is_none() {
                        options.output_path = Some(arg.clone());
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
        Err(error) => None,
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