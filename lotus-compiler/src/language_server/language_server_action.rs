use crate::{program::ProgramContext, command_line::CommandLineOptions};
use super::{validate, prepare_rename, provide_rename_edits};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum LanguageServerAction {
    Validate,
    PrepareRename,
    ProvideRenameEdits
}

impl LanguageServerAction {
    pub fn from_command_line_arg(option: &str) -> Option<Self> {
        match option {
            "--validate" => Some(Self::Validate),
            "--prepare-rename" => Some(Self::PrepareRename),
            "--provide-rename-edits" => Some(Self::ProvideRenameEdits),
            _ => None
        }
    }

    pub fn get_associated_callback(&self) -> (fn(&mut ProgramContext, options: &CommandLineOptions) -> Vec<String>) {
        match self {
            LanguageServerAction::Validate => validate,
            LanguageServerAction::PrepareRename => prepare_rename,
            LanguageServerAction::ProvideRenameEdits => provide_rename_edits,
        }
    }
}