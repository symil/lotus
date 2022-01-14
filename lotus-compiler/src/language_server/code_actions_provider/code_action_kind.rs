// https://code.visualstudio.com/api/references/vscode-api#CodeActionKind
pub enum CodeActionKind {
    Empty,
    QuickFix,
    Refactor,
    RefactorExtract,
    RefactorInline,
    RefactorRewrite,
    Source,
    SourceFixAll,
    SourceOrganizeImports,
}

impl CodeActionKind {
    pub fn to_str(&self) -> &'static str {
        match &self {
            CodeActionKind::Empty => "empty",
            CodeActionKind::QuickFix => "quick-fix",
            CodeActionKind::Refactor => "refactor",
            CodeActionKind::RefactorExtract => "refactor-extract",
            CodeActionKind::RefactorInline => "refactor-inline",
            CodeActionKind::RefactorRewrite => "refactor-rewrite",
            CodeActionKind::Source => "source",
            CodeActionKind::SourceFixAll => "source-fix-all",
            CodeActionKind::SourceOrganizeImports => "source-organize-imports",
        }
    }
}