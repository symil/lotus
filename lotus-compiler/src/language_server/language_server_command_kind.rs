use crate::{program::ProgramContext, command_line::CommandLineOptions};
use super::{validate, prepare_rename, provide_rename_edits, LanguageServerCommandParameters, provide_definition, provide_hover, provide_completion_items, LanguageServerCommandOutput};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum LanguageServerCommandKind {
    Validate,
    PrepareRename,
    ProvideRenameEdits,
    ProvideDefinition,
    ProvideHover,
    ProvideCompletionItems,
}

pub struct LanguageServerCommandCallbackDetails {
    pub force_reset: bool,
    pub callback: fn(&LanguageServerCommandParameters, &ProgramContext, &mut LanguageServerCommandOutput),
}

impl LanguageServerCommandKind {
    pub fn from_str(string: &str) -> Option<Self> {
        match string {
            "validate" => Some(Self::Validate),
            "prepare-rename" => Some(Self::PrepareRename),
            "provide-rename-edits" => Some(Self::ProvideRenameEdits),
            "provide-definition" => Some(Self::ProvideDefinition),
            "provide-hover" => Some(Self::ProvideHover),
            "provide-completion-items" => Some(Self::ProvideCompletionItems),
            _ => None
        }
    }

    pub fn get_callback_details(&self) -> LanguageServerCommandCallbackDetails {
        match self {
            LanguageServerCommandKind::Validate => LanguageServerCommandCallbackDetails {
                force_reset: true,
                callback: validate,
            },
            LanguageServerCommandKind::PrepareRename => LanguageServerCommandCallbackDetails {
                force_reset: false,
                callback: prepare_rename,
            },
            LanguageServerCommandKind::ProvideRenameEdits => LanguageServerCommandCallbackDetails {
                force_reset: false,
                callback: provide_rename_edits,
            },
            LanguageServerCommandKind::ProvideDefinition => LanguageServerCommandCallbackDetails {
                force_reset: false,
                callback: provide_definition,
            },
            LanguageServerCommandKind::ProvideHover => LanguageServerCommandCallbackDetails {
                force_reset: false,
                callback: provide_hover,
            },
            LanguageServerCommandKind::ProvideCompletionItems => LanguageServerCommandCallbackDetails {
                force_reset: true,
                callback: provide_completion_items,
            },
        }
    }
}