use crate::{program::ProgramContext, command_line::CommandLineOptions};
use super::{validate, prepare_rename, provide_rename_edits, LanguageServerCommandParameters, provide_definition, provide_hover};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum LanguageServerCommand {
    Validate,
    PrepareRename,
    ProvideRenameEdits,
    ProvideDefinition,
    ProvideHover,
}

pub struct LanguageServerCommandContent {
    pub force_init: bool,
    pub callback: fn(&LanguageServerCommandParameters, &ProgramContext, &mut Vec<String>),
}

impl LanguageServerCommand {
    pub fn from_str(string: &str) -> Option<Self> {
        match string {
            "validate" => Some(Self::Validate),
            "prepare-rename" => Some(Self::PrepareRename),
            "provide-rename-edits" => Some(Self::ProvideRenameEdits),
            "provide-definition" => Some(Self::ProvideDefinition),
            "provide-hover" => Some(Self::ProvideHover),
            _ => None
        }
    }

    pub fn get_content(&self) -> LanguageServerCommandContent {
        match self {
            LanguageServerCommand::Validate => LanguageServerCommandContent {
                force_init: true,
                callback: validate,
            },
            LanguageServerCommand::PrepareRename => LanguageServerCommandContent {
                force_init: false,
                callback: prepare_rename,
            },
            LanguageServerCommand::ProvideRenameEdits => LanguageServerCommandContent {
                force_init: false,
                callback: provide_rename_edits,
            },
            LanguageServerCommand::ProvideDefinition => LanguageServerCommandContent {
                force_init: false,
                callback: provide_definition,
            },
            LanguageServerCommand::ProvideHover => LanguageServerCommandContent {
                force_init: false,
                callback: provide_hover,
            },
        }
    }
}