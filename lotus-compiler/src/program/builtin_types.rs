use enum_iterator::IntoEnumIterator;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, IntoEnumIterator)]
pub enum BuiltinType {
    System,
    Void,
    Bool,
    Int,
    Float,
    Char,
    String,
    Pointer,
    Array,
    Function,
    Pair,
    Set,
    Map,
    DisplaySize,
    View,
    Event,
    EventOutput,
}

impl BuiltinType {
    pub fn get_name(&self) -> &'static str {
        match self {
            BuiltinType::System => "system",
            BuiltinType::Void => "void",
            BuiltinType::Bool => "bool",
            BuiltinType::Int => "int",
            BuiltinType::Float => "float",
            BuiltinType::Char => "char",
            BuiltinType::String => "string",
            BuiltinType::Pointer => "Pointer",
            BuiltinType::Array => "Array",
            BuiltinType::Function => "Function",
            BuiltinType::Pair => "Pair",
            BuiltinType::Set => "Set",
            BuiltinType::Map => "Map",
            BuiltinType::DisplaySize => "DisplaySize",
            BuiltinType::View => "View",
            BuiltinType::Event => "Event",
            BuiltinType::EventOutput => "EventOutput",
        }
    }
}