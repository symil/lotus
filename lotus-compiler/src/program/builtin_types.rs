use enum_iterator::IntoEnumIterator;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, IntoEnumIterator)]
pub enum BuiltinType {
    Any,
    System,
    Void,
    Bool,
    Int,
    Float,
    Char,
    String,
    Pointer,
    Array,
    Enum,
    Object,
    Function,
    Pair,
    Set,
    Map,
    DisplaySize,
    Event,
    EventOutput,
    Color,
}

impl BuiltinType {
    pub fn get_name(&self) -> &'static str {
        match self {
            BuiltinType::Any => "any",
            BuiltinType::System => "system",
            BuiltinType::Void => "void",
            BuiltinType::Bool => "bool",
            BuiltinType::Int => "int",
            BuiltinType::Float => "float",
            BuiltinType::Char => "char",
            BuiltinType::String => "string",
            BuiltinType::Pointer => "Pointer",
            BuiltinType::Array => "Array",
            BuiltinType::Enum => "Enum",
            BuiltinType::Object => "Object",
            BuiltinType::Function => "Function",
            BuiltinType::Pair => "Pair",
            BuiltinType::Set => "Set",
            BuiltinType::Map => "Map",
            BuiltinType::DisplaySize => "DisplaySize",
            BuiltinType::Event => "Event",
            BuiltinType::EventOutput => "EventOutput",
            BuiltinType::Color => "Color",
        }
    }
}