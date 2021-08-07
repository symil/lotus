pub const MEM_INIT_FUNC_NAME : &'static str = "mem_init";
pub const MEM_ALLOC_FUNC_NAME : &'static str = "mem_alloc";
pub const MEM_FREE_FUNC_NAME : &'static str = "mem_free";

pub const ARRAY_ALLOC_FUNC_NAME : &'static str = "array_alloc";
pub const ARRAY_LENGTH_FUNC_NAME : &'static str = "array_length";

// get stack order: self, index
pub const ARRAY_GET_I32_FUNC_NAME : &'static str = "array_get_i32";
pub const ARRAY_GET_F32_FUNC_NAME : &'static str = "array_get_f32";

// set order: value, self, index
pub const ARRAY_SET_I32_FUNC_NAME : &'static str = "array_set_i32";
pub const ARRAY_SET_F32_FUNC_NAME : &'static str = "array_set_f32";

pub const OBJECT_ALLOC_FUNC_NAME : &'static str = "object_alloc";

pub const LOG_I32_FUNC_NAME : &'static str = "log_i32";

pub const SELF_VAR_NAME : &'static str = "self";
pub const PAYLOAD_VAR_NAME : &'static str = "payload";

pub const NULL_ADDR : i32 = 0;