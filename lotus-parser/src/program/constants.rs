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

pub const DUMMY_FUNC_NAME : &'static str = "dummy";
pub const DUPLICATE_INT_WASM_FUNC_NAME : &'static str = "dup_i32";
pub const SWAP_INT_INT_WASM_FUNC_NAME : &'static str = "swap_i32_i32";
pub const SWAP_FLOAT_INT_WASM_FUNC_NAME : &'static str = "swap_f32_i32";

pub const INIT_GLOBALS_FUNC_NAME : &'static str = "init_globals";
pub const INIT_TYPES_FUNC_NAME : &'static str = "init_types";
pub const ENTRY_POINT_FUNC_NAME : &'static str = "__entry_point";
pub const THIS_VAR_NAME : &'static str = "this";
pub const PAYLOAD_VAR_NAME : &'static str = "__payload";
pub const RESULT_VAR_NAME : &'static str = "__fn_result";
pub const INIT_TYPE_METHOD_NAME : &'static str = "__init";

pub const OBJECT_TYPE_NAME : &'static str = "Object";
pub const THIS_TYPE_NAME : &'static str = "This";

pub const ITERABLE_ASSOCIATED_TYPE_NAME : &'static str = "Item";
pub const CREATE_METHOD_NAME : &'static str = "__create";
pub const STORE_FUNC_NAME : &'static str = "__store";
pub const LOAD_FUNC_NAME : &'static str = "__load";
pub const DEFAULT_METHOD_NAME : &'static str = "default";
pub const BUILTIN_DEFAULT_METHOD_NAME : &'static str = "__default";
pub const SET_CHAR_FUNC_NAME : &'static str = "__set_char";
pub const GET_BODY_FUNC_NAME : &'static str = "body";
pub const NONE_METHOD_NAME : &'static str = "__none";
pub const IS_NONE_FUNC_NAME : &'static str = "__is_none";
pub const IS_METHOD_NAME : &'static str = "__is";
pub const DESERIALIZE_METHOD_NAME : &'static str = "__deserialize";
pub const DESERIALIZE_DYN_METHOD_NAME : &'static str = "__deserialize_dyn";
pub const UNWRAP_FUNC_NAME : &'static str = "unwrap";

pub const GET_AT_INDEX_FUNC_NAME : &'static str = "get_at";
pub const SET_AT_INDEX_FUNC_NAME : &'static str = "set_at";
pub const GET_ITERABLE_LEN_FUNC_NAME : &'static str = "get_iterable_len";
pub const GET_ITERABLE_PTR_FUNC_NAME : &'static str = "get_iterable_ptr";

pub const NAN_WASM : &'static str = "nan:0x200000";

pub const TYPE_ID_TO_ANCESTOR_IDS_GLOBAL_NAME : &'static str = "TYPE_ID_TO_ANCESTOR_IDS";