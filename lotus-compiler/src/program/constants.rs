use super::{BuiltinInterface, BuiltinType};

pub const WASM_PAGE_BYTE_SIZE : usize = 2usize.pow(16); // 64 KiB
pub const MEMORY_CELL_BYTE_SIZE : usize = 4;

pub const NULL_ADDR : i32 = 0;
pub const HEADER_MEMORY_WASM_PAGE_COUNT : usize = 1;
pub const MAX_VIRTUAL_PAGE_COUNT_PER_BLOCK_SIZE : usize = 64;
pub const VIRTUAL_PAGE_SIZE_COUNT : usize = 8;
pub const MEMORY_METADATA_SIZE : usize = MAX_VIRTUAL_PAGE_COUNT_PER_BLOCK_SIZE * VIRTUAL_PAGE_SIZE_COUNT * MEMORY_CELL_BYTE_SIZE;

pub const GENERATED_METHODS_TABLE_START : usize = MEMORY_METADATA_SIZE;
pub const GENERATED_METHOD_COUNT_PER_TYPE : usize = 4; // log, retain, serialize, deserialize

pub const INT_NONE_VALUE : i32 = i32::MIN;
pub const NONE_LITERAL : &'static str = "none";

pub const INIT_GLOBALS_FUNC_NAME : &'static str = "init_globals";
pub const INIT_TYPES_FUNC_NAME : &'static str = "init_types";
pub const INIT_EVENTS_FUNC_NAME : &'static str = "init_events";
pub const RETAIN_GLOBALS_FUNC_NAME : &'static str = "retain_globals";
pub const ENTRY_POINT_FUNC_NAME : &'static str = "__entry_point";
pub const THIS_VAR_NAME : &'static str = "this";
pub const CLOSURE_VARIABLES_VAR_NAME : &'static str = "closure_args";
pub const INIT_TYPE_METHOD_NAME : &'static str = "__init";
pub const END_INIT_TYPE_METHOD_NAME : &'static str = "__end_init";

pub const OBJECT_TYPE_NAME : &'static str = "Object";
pub const ENUM_TYPE_NAME : &'static str = "Enum";
pub const THIS_TYPE_NAME : &'static str = "This";
pub const ITERABLE_ASSOCIATED_TYPE_NAME : &'static str = "Item";

pub const TUPLE_FIRST_ASSOCIATED_TYPE_NAME : &'static str = "First";
pub const TUPLE_SECOND_ASSOCIATED_TYPE_NAME : &'static str = "Second";
pub const TUPLE_FIRST_METHOD_NAME : &'static str = "first";
pub const TUPLE_SECOND_METHOD_NAME : &'static str = "second";

pub const NEW_METHOD_NAME : &'static str = "new";
pub const CREATE_METHOD_NAME : &'static str = "__create";
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
pub const MEM_ALLOC_FUNC_NAME : &'static str = "mem_alloc";

pub const GET_AT_INDEX_FUNC_NAME : &'static str = "get_at";
pub const SET_AT_INDEX_FUNC_NAME : &'static str = "set_at";
pub const GET_ITERABLE_LEN_FUNC_NAME : &'static str = "get_iterable_len";
pub const GET_ITERABLE_PTR_FUNC_NAME : &'static str = "get_iterable_ptr";

pub const TYPE_ID_TO_ANCESTOR_IDS_GLOBAL_NAME : &'static str = "TYPE_ID_TO_ANCESTOR_IDS";
pub const EVENT_HOOKS_GLOBAL_NAME : &'static str = "EVENT_HOOKS";
pub const BEFORE_EVENT_CALLBACKS_GLOBAL_NAME : &'static str = "BEFORE_EVENT_CALLBACKS";
pub const AFTER_EVENT_CALLBACKS_GLOBAL_NAME : &'static str = "AFTER_EVENT_CALLBACKS";
pub const INSERT_EVENT_CALLBACK_FUNC_NAME : &'static str = "insert_event_callback";

pub const NAN_WASM : &'static str = "nan:0x200000";

pub const EXPORTED_FUNCTIONS : &'static [&'static str] = &[
    "main",
    "start_client",
    "update_client",
    "start_server",
    "update_server",
];