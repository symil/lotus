#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BuiltinType {
    System,
    Bool,
    Int,
    Float,
    String,
    Pointer,
    Array,
}