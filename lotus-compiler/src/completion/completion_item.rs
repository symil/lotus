use super::CompletionItemKind;

pub struct CompletionItem {
    // What will be inserted in the editor
    pub label: String,
    // Icon indicating the kind of item
    pub kind: Option<CompletionItemKind>,
    // Displayed in smaller character, at the right of the label
    pub description: Option<String>,
    // Title of the right panel
    pub detail: Option<String>,
    // Content of the right panel
    pub documentation: Option<String>
}

impl CompletionItem {
    pub fn new(label: String) -> Self {
        Self {
            label,
            kind: None,
            description: None,
            detail: None,
            documentation: None,
        }
    }

    pub fn kind(mut self, kind: CompletionItemKind) -> Self {
        self.kind = Some(kind);
        self
    }

    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn detail(mut self, detail: String) -> Self {
        self.detail = Some(detail);
        self
    }

    pub fn documentation(mut self, documentation: String) -> Self {
        self.documentation = Some(documentation);
        self
    }
}