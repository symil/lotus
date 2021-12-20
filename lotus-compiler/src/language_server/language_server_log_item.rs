use crate::program::CompilationError;

pub enum LanguegeServerLogItem {
    File(String),
    Error(CompilationError)
}

impl LanguegeServerLogItem {
    pub fn to_string(&self) -> Option<String> {
        match self {
            LanguegeServerLogItem::File(file_path) => Some(format!("file;{}", file_path)),
            LanguegeServerLogItem::Error(error) => error.get_message().map(|message| format!("error;{};{};{}", error.location.start, error.location.end, message)),
        }
    }
}