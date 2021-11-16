#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BuiltinInterface {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Shl,
    Shr,
    And,
    Or,
    Xor,
    Ge,
    Gt,
    Le,
    Lt,
    Plus,
    Minus,
    Not,
    ToBool,
    GetAtIndex,
    SetAtIndex,
    Iterable,
    Unwrap,
    Builtin,
    Tuple,
    Object,
    World
}

pub const DEFAULT_INTERFACES : &'static[BuiltinInterface] = &[
    BuiltinInterface::Builtin,
];

impl BuiltinInterface {
    pub fn get_name(&self) -> &'static str {
        match self {
            BuiltinInterface::Add => "Add",
            BuiltinInterface::Sub => "Sub",
            BuiltinInterface::Mul => "Mul",
            BuiltinInterface::Div => "Div",
            BuiltinInterface::Mod => "Mod",
            BuiltinInterface::Shl => "Shl",
            BuiltinInterface::Shr => "Shr",
            BuiltinInterface::And => "And",
            BuiltinInterface::Or => "Or",
            BuiltinInterface::Xor => "Xor",
            BuiltinInterface::Ge => "Ge",
            BuiltinInterface::Gt => "Gt",
            BuiltinInterface::Le => "Le",
            BuiltinInterface::Lt => "Lt",
            BuiltinInterface::Plus => "Plus",
            BuiltinInterface::Minus => "Minus",
            BuiltinInterface::Not => "Not",
            BuiltinInterface::ToBool => "ToBool",
            BuiltinInterface::GetAtIndex => "GetAtIndex",
            BuiltinInterface::SetAtIndex => "SetAtIndex",
            BuiltinInterface::Iterable => "Iterable",
            BuiltinInterface::Unwrap => "Unwrap",
            BuiltinInterface::Builtin => "Builtin",
            BuiltinInterface::Tuple => "Tuple",
            BuiltinInterface::Object => "Object",
            BuiltinInterface::World => "World",
        }
    }
}