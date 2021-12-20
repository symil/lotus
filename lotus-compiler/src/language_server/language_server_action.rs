use crate::{program::ProgramContext, command_line::CommandLineOptions};

use super::{validate_program, LanguegeServerLogItem};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum LanguageServerAction {
    Validate,
}

impl LanguageServerAction {
    pub fn from_command_line_arg(option: &str) -> Option<Self> {
        match option {
            "--validate" => Some(Self::Validate),
            _ => None
        }
    }

    pub fn get_associated_callback(&self) -> (fn(&mut ProgramContext, options: &CommandLineOptions) -> Vec<LanguegeServerLogItem>) {
        match self {
            LanguageServerAction::Validate => validate_program,
        }
    }
}