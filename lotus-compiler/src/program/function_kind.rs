#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionKind {
    Standard,
    DefaultValue,
    EventCallback
}