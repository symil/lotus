#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BuiltinType {
    System,
    Bool,
    Int,
    Float,
    Char,
    String,
    Pointer,
    Array,
    DisplaySize
}

impl BuiltinType {
    pub fn get_name(&self) -> &'static str {
        match self {
            BuiltinType::System => "system",
            BuiltinType::Bool => "bool",
            BuiltinType::Int => "int",
            BuiltinType::Float => "float",
            BuiltinType::Char => "char",
            BuiltinType::String => "string",
            BuiltinType::Pointer => "Pointer",
            BuiltinType::Array => "Array",
            BuiltinType::DisplaySize => "DisplaySize",
        }
    }
}