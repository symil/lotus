use super::{BuiltinInterface, BuiltinType};

pub const SOURCE_FILE_EXTENSION : &'static str = "lt";
pub const COMMENT_START_TOKEN : &'static str = "//";
pub const PRELUDE_NAMESPACE : &'static str = "std";
pub const SELF_NAMESPACE : &'static str = "self";

pub const WASM_PAGE_BYTE_SIZE : usize = 2usize.pow(16); // 64 KiB
pub const MEMORY_CELL_BYTE_SIZE : usize = 4;

pub const NULL_ADDR : i32 = 0;
pub const HEADER_MEMORY_WASM_PAGE_COUNT : usize = 1;
pub const MAX_VIRTUAL_PAGE_COUNT_PER_BLOCK_SIZE : usize = 64;
pub const VIRTUAL_PAGE_SIZE_COUNT : usize = 8;
pub const MEMORY_METADATA_SIZE : usize = MAX_VIRTUAL_PAGE_COUNT_PER_BLOCK_SIZE * VIRTUAL_PAGE_SIZE_COUNT * MEMORY_CELL_BYTE_SIZE;

pub const OBJECT_HEADER_SIZE : usize = 1;

pub const INT_NONE_VALUE : i32 = i32::MIN;
pub const NONE_LITERAL : &'static str = "none";

pub const INIT_GLOBALS_FUNC_NAME : &'static str = "init_globals";
pub const INIT_TYPES_FUNC_NAME : &'static str = "init_types";
pub const INIT_EVENTS_FUNC_NAME : &'static str = "init_events";
pub const RETAIN_GLOBALS_FUNC_NAME : &'static str = "retain_globals";
pub const ENTRY_POINT_FUNC_NAME : &'static str = "__entry_point";
pub const SELF_VAR_NAME : &'static str = "self";
pub const CLOSURE_VARIABLES_VAR_NAME : &'static str = "closure_args";
pub const INIT_TYPE_METHOD_NAME : &'static str = "__init";
pub const END_INIT_TYPE_METHOD_NAME : &'static str = "__end_init";

pub const OBJECT_TYPE_NAME : &'static str = "Object";
pub const ENUM_TYPE_NAME : &'static str = "Enum";
pub const SELF_TYPE_NAME : &'static str = "Self";
pub const ITERABLE_ASSOCIATED_TYPE_NAME : &'static str = "Item";

pub const TUPLE_FIRST_ASSOCIATED_TYPE_NAME : &'static str = "First";
pub const TUPLE_SECOND_ASSOCIATED_TYPE_NAME : &'static str = "Second";
pub const TUPLE_FIRST_METHOD_NAME : &'static str = "first";
pub const TUPLE_SECOND_METHOD_NAME : &'static str = "second";

pub const NEW_METHOD_NAME : &'static str = "new";
pub const OBJECT_CREATE_METHOD_NAME : &'static str = "__create";
pub const ARRAY_CREATE_METHOD_NAME : &'static str = "with_capacity";
pub const STRING_CREATE_METHOD_NAME : &'static str = "__create";
pub const PUSH_UNCHECKED_METHOD_NAME : &'static str = "push_unchecked";
pub const STORE_FUNC_NAME : &'static str = "__store";
pub const LOAD_FUNC_NAME : &'static str = "__load";
pub const DEFAULT_METHOD_NAME : &'static str = "default";
pub const BUILTIN_DEFAULT_METHOD_NAME : &'static str = "__default";
pub const SET_CHAR_FUNC_NAME : &'static str = "__set_char";
pub const GET_BODY_FUNC_NAME : &'static str = "body";
pub const NONE_METHOD_NAME : &'static str = "__none";
pub const IS_NONE_METHOD_NAME : &'static str = "__is_none";
pub const IS_METHOD_NAME : &'static str = "__is";
pub const DESERIALIZE_METHOD_NAME : &'static str = "__deserialize";
pub const DESERIALIZE_DYN_METHOD_NAME : &'static str = "__deserialize_dyn";
pub const RETAIN_METHOD_NAME : &'static str = "__retain";
pub const TO_STRING_METHOD_NAME : &'static str = "to_string";
pub const MEM_ALLOC_FUNC_NAME : &'static str = "alloc_memory";

pub const GET_AT_INDEX_FUNC_NAME : &'static str = "__get_at_index";
pub const SET_AT_INDEX_FUNC_NAME : &'static str = "__set_at_index";
pub const GET_ITERABLE_LEN_FUNC_NAME : &'static str = "__get_iterable_len";
pub const GET_ITERABLE_PTR_FUNC_NAME : &'static str = "__get_iterable_ptr";

pub const EVENT_VAR_NAME : &'static str = "evt";
pub const EVENT_OUTPUT_VAR_NAME : &'static str = "__output";
pub const HAS_TARGET_METHOD_NAME : &'static str = "has_target";
pub const TYPE_ID_TO_ANCESTOR_IDS_GLOBAL_NAME : &'static str = "TYPE_ID_TO_ANCESTOR_IDS";
pub const EVENT_CALLBACKS_GLOBAL_NAME : &'static str = "EVENT_CALLBACKS";
pub const INSERT_EVENT_CALLBACK_FUNC_NAME : &'static str = "insert_event_callback";
pub const SORT_EVENT_CALLBACK_FUNC_NAME : &'static str = "sort_event_callbacks";

pub const NAN_WASM : &'static str = "nan:0x200000";

pub const EXPORTED_FUNCTIONS : &'static [&'static str] = &[
    "main",
    "start_client",
    "update_client",
    "start_server",
    "update_server",
];