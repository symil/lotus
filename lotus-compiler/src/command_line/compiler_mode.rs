#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum CompilerMode {
    Compile,
    Validate,
}

impl CompilerMode {
    pub fn from_command_line_arg(option: &str) -> Option<Self> {
        match option {
            "--compile" => Some(Self::Compile),
            "--validate" => Some(Self::Validate),
            _ => None
        }
    }
}