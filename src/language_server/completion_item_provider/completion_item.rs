use crate::language_server::Range;
use super::{CompletionItemKind, CompletionItemPosition, CompletionItemCommand};

// https://code.visualstudio.com/api/references/vscode-api#CompletionItem
pub struct CompletionItem {
    // What will be inserted in the editor
    pub label: String,
    // Where the item will be positionned relative to others
    pub position: Option<CompletionItemPosition>,
    // Icon indicating the kind of item
    pub kind: Option<CompletionItemKind>,
    // Range of the text that should be replaced
    pub range: Option<Range>,
    // Displayed in smaller character, at the right of the label
    pub description: Option<String>,
    // Title of the right panel
    pub detail: Option<String>,
    // Content of the right panel
    pub documentation: Option<String>,
    // What will be inserted in the document
    pub insert_text: Option<String>,
    // What will be used when filtering the item
    pub filter_text: Option<String>,
    // What will be used when sorting the item relative to others (complementary to `position`)
    pub sort_text: Option<String>,
    // Command that will be run after the completion is done
    pub command: Option<CompletionItemCommand>
}

impl CompletionItem {
    pub fn new(label: String) -> Self {
        Self {
            label,
            position: None,
            kind: None,
            range: None,
            description: None,
            detail: None,
            documentation: None,
            insert_text: None,
            filter_text: None,
            sort_text: None,
            command: None,
        }
    }
}