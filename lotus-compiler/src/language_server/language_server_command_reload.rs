#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LanguageServerCommandReload {
    No,
    Yes,
    WithHook
}