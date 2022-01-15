// https://code.visualstudio.com/api/references/vscode-api#CodeActionKind
#[derive(Debug, Clone, Copy, PartialEq)]
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

impl ToString for CodeActionKind {
    fn to_string(&self) -> String {
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
        }.to_string()
    }
}