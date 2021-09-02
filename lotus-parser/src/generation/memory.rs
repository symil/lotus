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