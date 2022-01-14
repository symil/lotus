use crate::{program::ProgramContext, command_line::CommandLineOptions, language_server::{validate, prepare_rename, provide_rename_edits, provide_definition, provide_hover, provide_completion_items, provide_signature_help}};
use super::{LanguageServerCommandParameters, LanguageServerCommandOutput, LanguageServerCommandReload};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum LanguageServerCommandKind {
    Validate,
    PrepareRename,
    ProvideRenameEdits,
    ProvideDefinition,
    ProvideHover,
    ProvideCompletionItems,
    ProvideSignatureHelp,
}

pub type LanguageServerCommandCallback = fn(&LanguageServerCommandParameters, &ProgramContext, &mut LanguageServerCommandOutput);

impl LanguageServerCommandKind {
    pub fn from_str(string: &str) -> Option<Self> {
        match string {
            "validate" => Some(Self::Validate),
            "prepare-rename" => Some(Self::PrepareRename),
            "provide-rename-edits" => Some(Self::ProvideRenameEdits),
            "provide-definition" => Some(Self::ProvideDefinition),
            "provide-hover" => Some(Self::ProvideHover),
            "provide-completion-items" => Some(Self::ProvideCompletionItems),
            "provide-signature-help" => Some(Self::ProvideSignatureHelp),
            _ => None
        }
    }

    pub fn get_callback(&self) -> LanguageServerCommandCallback {
        match self {
            LanguageServerCommandKind::Validate => validate,
            LanguageServerCommandKind::PrepareRename => prepare_rename,
            LanguageServerCommandKind::ProvideRenameEdits => provide_rename_edits,
            LanguageServerCommandKind::ProvideDefinition => provide_definition,
            LanguageServerCommandKind::ProvideHover => provide_hover,
            LanguageServerCommandKind::ProvideCompletionItems => provide_completion_items,
            LanguageServerCommandKind::ProvideSignatureHelp => provide_signature_help,
        }
    }
}