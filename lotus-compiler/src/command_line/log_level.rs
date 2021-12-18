#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    Silent,
    Short,
    Detailed
}

impl Default for LogLevel {
    fn default() -> Self {
        Self::Short
    }
}

impl LogLevel {
    pub fn from_command_line_arg(string: &str) -> Option<Self> {
        match string {
            "--details" => Some(Self::Detailed),
            "--silent" => Some(Self::Silent),
            _ => None
        }
    }
}