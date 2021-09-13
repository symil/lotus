use super::{BuiltinInterface, BuiltinType};

pub const WASM_PAGE_BYTE_SIZE : usize = 2usize.pow(16); // 64 KiB
pub const NULL_ADDR : i32 = 0;
pub const VALUE_BYTE_SIZE : usize = 4;
pub const HEADER_MEMORY_WASM_PAGE_COUNT : usize = 1;
pub const MAX_VIRTUAL_PAGE_COUNT_PER_BLOCK_SIZE : usize = 64;
pub const VIRTUAL_PAGE_SIZE_COUNT : usize = 8;
pub const MEMORY_METADATA_SIZE : usize = MAX_VIRTUAL_PAGE_COUNT_PER_BLOCK_SIZE * VIRTUAL_PAGE_SIZE_COUNT * VALUE_BYTE_SIZE;

pub const GENERATED_METHODS_TABLE_START : usize = MEMORY_METADATA_SIZE;
pub const GENERATED_METHOD_COUNT_PER_TYPE : usize = 4; // log, retain, serialize, deserialize

pub const MEMORY_ALLOC_FUNC_NAME : &'static str = "__mem_alloc";
pub const MEMORY_FREE_FUNC_NAME : &'static str = "__mem_free";
pub const MEMORY_COPY_FUNC_NAME : &'static str = "__mem_copy";
pub const MEMORY_RETAIN_FUNC_NAME : &'static str = "__mem_retain";
pub const MEMORY_RETAIN_OBJECT_FUNC_NAME : &'static str = "__mem_retain_object";
pub const MEMORY_GARBAGE_COLLECT_FUNC_NAME : &'static str = "__trigger_garbage_collection";

pub const INIT_GLOBALS_FUNC_NAME : &'static str = "__init_globals";
pub const ENTRY_POINT_FUNC_NAME : &'static str = "__entry_point";
pub const THIS_VAR_NAME : &'static str = "__this";
pub const PAYLOAD_VAR_NAME : &'static str = "__payload";
pub const RESULT_VAR_NAME : &'static str = "__fn_result";

pub const THIS_TYPE_NAME : &'static str = "This";

pub const BUILTIN_TYPES : &'static[(BuiltinType, &'static str)] = &[
    (BuiltinType::Bool, "bool"),
    (BuiltinType::Int, "int"),
    (BuiltinType::Float, "float"),
    (BuiltinType::String, "string"),
    (BuiltinType::Pointer, "Pointer"),
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
    (BuiltinInterface::Not, "Not",  "not"),
    (BuiltinInterface::Plus, "Plus",  "plus"),
    (BuiltinInterface::Minus, "Minus",  "minus"),
    (BuiltinInterface::ToBool, "ToBool",  "to_bool"),
    (BuiltinInterface::GetAtIndex, "GetAtIndex",  "get_at_index"),
    (BuiltinInterface::SetAtIndex, "SetAtIndex",  "set_at_index"),
    (BuiltinInterface::Iterable, "Iterable",  ""),
];

pub const NEW_FUNC_NAME : &'static str = "__new";
pub const DEFAULT_FUNC_NAME : &'static str = "__default";
pub const SET_CHAR_FUNC_NAME : &'static str = "__set_char";
pub const GET_BODY_FUNC_NAME : &'static str = "__get_body";
pub const GET_AS_PTR_METHOD_NAME : &'static str = "__get_as_ptr";
pub const SET_AS_PTR_METHOD_NAME : &'static str = "__set_as_ptr"; // static method, argument order: value, pointer, index

pub const GET_AT_INDEX_FUNC_NAME : &'static str = "get_at_index";
pub const SET_AT_INDEX_FUNC_NAME : &'static str = "set_at_index";
pub const GET_ITERABLE_PTR_FUNC_NAME : &'static str = "get_iterable_ptr";
pub const GET_ITERABLE_LEN_FUNC_NAME : &'static str = "get_iterable_len";