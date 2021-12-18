use std::path::PathBuf;
use crate::program::{SourceDirectoryDetails, PRELUDE_NAMESPACE, SELF_NAMESPACE};
use super::{LogLevel, CARGO_MANIFEST_DIR_PATH, PRELUDE_DIR_NAME, CompilerMode};

pub struct CommandLineOptions {
    pub prelude_path: String,
    pub input_path: String,
    pub output_path: String,
    pub log_level: LogLevel,
    pub mode: CompilerMode
}

impl CommandLineOptions {
    pub fn parse_from_args(args: Vec<String>) -> Option<Self> {
        let mut options = Self {
            prelude_path: get_default_prelude_path(),
            input_path: String::new(),
            output_path: String::new(),
            log_level: LogLevel::Short,
            mode: CompilerMode::Compile
        };

        for arg in &args {
            match is_option(arg) {
                true => {
                    if let Some(mode) = CompilerMode::from_command_line_arg(arg) {
                        options.mode = mode;
                    } else if let Some(log_level) = LogLevel::from_command_line_arg(arg) {
                        options.log_level = log_level;
                    }
                },
                false => {
                    if options.input_path.is_empty() {
                        options.input_path = arg.to_string();
                    } else if options.output_path.is_empty() {
                        options.output_path = arg.to_string();
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
        // TODO: check that the paths are actually valid?
        !self.input_path.is_empty() && !self.output_path.is_empty()
    }

    pub fn get_source_directories(&self) -> Vec<SourceDirectoryDetails> {
        let mut result = vec![];

        result.push(SourceDirectoryDetails {
            path: self.prelude_path.clone()
        });

        result.push(SourceDirectoryDetails {
            path: self.input_path.clone()
        });

        result
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