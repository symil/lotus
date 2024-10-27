use enum_iterator::Sequence;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Sequence)]
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
    Color,
    // Event,
    EventOptions,
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
            // BuiltinType::Event => "Event",
            BuiltinType::EventOptions => "EventOptions",
            BuiltinType::Color => "Color",
        }
    }
}