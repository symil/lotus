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

pub const MEMORY_ALLOC_FUNC_NAME : &'static str = "__mem_alloc";
pub const MEMORY_FREE_FUNC_NAME : &'static str = "__mem_free";
pub const MEMORY_COPY_FUNC_NAME : &'static str = "__mem_copy";
pub const MEMORY_RETAIN_FUNC_NAME : &'static str = "__mem_retain";
pub const MEMORY_RETAIN_OBJECT_FUNC_NAME : &'static str = "__mem_retain_object";
pub const MEMORY_GARBAGE_COLLECT_FUNC_NAME : &'static str = "__trigger_garbage_collection";

pub const INIT_GLOBALS_FUNC_NAME : &'static str = "__init_globals";
pub const ENTRY_POINT_FUNC_NAME : &'static str = "__entry_point";
pub const THIS_VAR_NAME : &'static str = "this";
pub const PAYLOAD_VAR_NAME : &'static str = "__payload";
pub const RESULT_VAR_NAME : &'static str = "__fn_result";

pub const OBJECT_TYPE_NAME : &'static str = "Object";
pub const THIS_TYPE_NAME : &'static str = "This";

pub const ITERABLE_ASSOCIATED_TYPE_NAME : &'static str = "Item";
pub const NEW_FUNC_NAME : &'static str = "__new";
pub const STORE_FUNC_NAME : &'static str = "__store";
pub const LOAD_FUNC_NAME : &'static str = "__load";
pub const DEFAULT_FUNC_NAME : &'static str = "__default";
pub const SET_CHAR_FUNC_NAME : &'static str = "__set_char";
pub const GET_BODY_FUNC_NAME : &'static str = "body";

pub const GET_AT_INDEX_FUNC_NAME : &'static str = "get_at";
pub const SET_AT_INDEX_FUNC_NAME : &'static str = "set_at";
pub const GET_ITERABLE_LEN_FUNC_NAME : &'static str = "get_iterable_len";
pub const GET_ITERABLE_PTR_FUNC_NAME : &'static str = "get_iterable_ptr";

pub const MACRO_TYPE_ID : &'static str = "TYPE_ID";
pub const MACRO_FIELD_COUNT : &'static str = "FIELD_COUNT";
pub const MACRO_FIELD_NAME : &'static str = "FIELD_NAME";
pub const MACRO_FIELD_TYPE : &'static str = "FIELD_TYPE";