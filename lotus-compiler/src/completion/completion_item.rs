use super::CompletionItemKind;

pub struct CompletionItem {
    pub name: String,
    pub kind: CompletionItemKind
}

impl CompletionItem {
    pub fn new(name: &str, kind: CompletionItemKind) -> Self {
        Self {
            name: name.to_string(),
            kind,
        }
    }
}