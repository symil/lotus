use super::CursorLocation;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProgramContextMode {
    Compile,
    Validate
}

#[derive(Debug, Clone)]
pub struct ProgramContextOptions {
    pub mode: ProgramContextMode,
    pub cursor_location: Option<CursorLocation>,
}

impl ProgramContextOptions {
    pub fn compile() -> Self {
        Self {
            mode: ProgramContextMode::Compile,
            cursor_location: None,
        }
    }

    pub fn validate() -> Self {
        Self {
            mode: ProgramContextMode::Validate,
            cursor_location: None,
        }
    }

    // pub fn language_server(root_directory_path: &str, file_path: &str, cursor_index: usize) -> Self {
    //     Self {
    //         mode: ProgramContextMode::Validate,
    //         cursor_location: Some(CursorLocation::new(root_directory_path, file_path, cursor_index)),
    //     }
    // }
}