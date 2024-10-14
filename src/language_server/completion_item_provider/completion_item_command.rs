#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum CompletionItemCommand {
    None,
    TriggerSignatureHelp,
    TriggerCompletion
}

impl ToString for CompletionItemCommand {
    fn to_string(&self) -> String {
        match self {
            CompletionItemCommand::None => "",
            CompletionItemCommand::TriggerSignatureHelp => "trigger-signature-help",
            CompletionItemCommand::TriggerCompletion => "trigger-completion",
        }.to_string()
    }
}