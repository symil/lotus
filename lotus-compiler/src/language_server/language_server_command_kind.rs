use crate::{program::ProgramContext, command_line::CommandLineOptions};
use super::{validate, prepare_rename, provide_rename_edits, LanguageServerCommandParameters, provide_definition, provide_hover, provide_completion_items, LanguageServerCommandOutput, LanguageServerCommandReload};

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
    pub reload: LanguageServerCommandReload,
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
                reload: LanguageServerCommandReload::Yes,
                callback: validate,
            },
            LanguageServerCommandKind::PrepareRename => LanguageServerCommandCallbackDetails {
                reload: LanguageServerCommandReload::No,
                callback: prepare_rename,
            },
            LanguageServerCommandKind::ProvideRenameEdits => LanguageServerCommandCallbackDetails {
                reload: LanguageServerCommandReload::No,
                callback: provide_rename_edits,
            },
            LanguageServerCommandKind::ProvideDefinition => LanguageServerCommandCallbackDetails {
                reload: LanguageServerCommandReload::No,
                callback: provide_definition,
            },
            LanguageServerCommandKind::ProvideHover => LanguageServerCommandCallbackDetails {
                reload: LanguageServerCommandReload::No,
                callback: provide_hover,
            },
            LanguageServerCommandKind::ProvideCompletionItems => LanguageServerCommandCallbackDetails {
                reload: LanguageServerCommandReload::WithHook,
                callback: provide_completion_items,
            },
        }
    }
}