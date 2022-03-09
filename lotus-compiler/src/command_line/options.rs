use std::{path::{PathBuf, Path}};
use crate::{program::{SourceDirectory, PRELUDE_NAMESPACE, SELF_NAMESPACE}, language_server::LanguageServerCommandKind};
use super::{LogLevel};

#[derive(Debug)]
pub struct CommandLineOptions {
    pub input_path: Option<String>,
    pub output_path: Option<String>,
    pub framework: Option<String>,
    pub log_level: LogLevel,
    pub validate: bool,
    pub run_as_server: bool,
    pub run_benchmark: bool,
    pub command: Option<String>,
}

impl CommandLineOptions {
    pub fn parse_from_args(args: Vec<String>) -> Self {
        let mut options = Self {
            input_path: None,
            output_path: None,
            framework: None,
            log_level: LogLevel::Short,
            validate: false,
            run_as_server: false,
            run_benchmark: false,
            command: None,
        };

        for arg in &args[1..] {
            match is_option(arg) {
                true => {
                    if let Some(log_level) = LogLevel::from_command_line_arg(arg) {
                        options.log_level = log_level;
                    } else if arg == "--validate" || arg == "-v" {
                        options.validate = true;
                    } else if arg == "--benchmark" {
                        options.run_benchmark = true;
                    } else if arg == "--server" {
                        options.run_as_server = true;
                    } else if let Some(command) = get_option_value(arg, "--command") {
                        options.command = Some(command);
                    } else if let Some(framework) = get_option_value(arg, "--framework") {
                        options.framework = Some(framework);
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

        if options.validate && options.output_path.is_none() {
            options.output_path = Some(String::new());
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

fn get_option_value(arg: &str, option_name: &str) -> Option<String> {
    if arg.starts_with(option_name) && arg.as_bytes().get(option_name.len()).unwrap_or(&0) == &(b'=') {
        return Some(arg[option_name.len() + 1..].to_string());
    }

    None
}