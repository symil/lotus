#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum PostCompletionCommand {
    None,
    TriggerSignatureHelp,
    TriggerCompletion
}

impl ToString for PostCompletionCommand {
    fn to_string(&self) -> String {
        match self {
            PostCompletionCommand::None => "",
            PostCompletionCommand::TriggerSignatureHelp => "trigger-signature-help",
            PostCompletionCommand::TriggerCompletion => "trigger-completion",
        }.to_string()
    }
}