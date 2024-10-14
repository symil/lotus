use crate::package::Package;
use super::CursorLocation;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinaryKind {
    Cli,
    App
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProgramContextMode {
    Compile(BinaryKind),
    Validate
}

#[derive(Debug, Clone)]
pub struct ProgramContextOptions {
    pub package: Package,
    pub mode: ProgramContextMode,
    pub cursor_location: Option<CursorLocation>,
}

impl ProgramContextOptions {
    pub fn is_compile_mode(&self) -> bool {
        match &self.mode {
            ProgramContextMode::Compile(_) => true,
            _ => false
        }
    }
}