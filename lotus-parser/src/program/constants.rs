use super::{BuiltinInterface, BuiltinType};

pub const INIT_GLOBALS_FUNC_NAME : &'static str = "__init_globals";
pub const ENTRY_POINT_FUNC_NAME : &'static str = "__entry_point";
pub const THIS_VAR_NAME : &'static str = "__this";
pub const PAYLOAD_VAR_NAME : &'static str = "__payload";
pub const RESULT_VAR_NAME : &'static str = "__fn_result";

pub const THIS_TYPE_NAME : &'static str = "This";
pub const PTR_GET_METHOD_NAME : &'static str = "__ptr_get";
pub const PTR_SET_METHOD_NAME : &'static str = "__ptr_set";

pub const BUILTIN_TYPES : &'static[(BuiltinType, &'static str)] = &[
    (BuiltinType::Bool, "bool"),
    (BuiltinType::Int, "int"),
    (BuiltinType::Float, "float"),
    (BuiltinType::String, "string"),
    (BuiltinType::Array, "Array"),
];

pub const BUILTIN_INTERFACES : &'static[(BuiltinInterface, &'static str, &'static str)] = &[
    (BuiltinInterface::Add, "Add", "add"),
    (BuiltinInterface::Sub, "Sub", "sub"),
    (BuiltinInterface::Mul, "Mul", "mul"),
    (BuiltinInterface::Div, "Div", "div"),
    (BuiltinInterface::Mod, "Mod", "mod"),
    (BuiltinInterface::Shl, "Shl", "shl"),
    (BuiltinInterface::Shr, "Shr", "shr"),
    (BuiltinInterface::And, "And", "and"),
    (BuiltinInterface::Or, "Or",  "or"),
    (BuiltinInterface::Eq, "Eq",  "eq"),
    (BuiltinInterface::Ne, "Ne",  "ne"),
    (BuiltinInterface::Ge, "Ge",  "ge"),
    (BuiltinInterface::Gt, "Gt",  "gt"),
    (BuiltinInterface::Le, "Le",  "le"),
    (BuiltinInterface::Lt, "Lt",  "lt"),
];
