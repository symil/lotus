pub const NULL_ADDR : i32 = 0;
pub const VALUE_BYTE_SIZE : usize = 4;
pub const HEADER_MEMORY_WASM_PAGE_COUNT : usize = 1;

pub const ARRAY_CONCAT_FUNC_NAME : &'static str = "array_concat";

pub const OBJECT_ALLOC_FUNC_NAME : &'static str = "object_alloc";

pub const THIS_VAR_NAME : &'static str = "__this";
pub const PAYLOAD_VAR_NAME : &'static str = "__payload";
pub const RESULT_VAR_NAME : &'static str = "__fn_result";
pub const INIT_GLOBALS_FUNC_NAME : &'static str = "__init_globals";
pub const ENTRY_POINT_FUNC_NAME : &'static str = "__entry_point";