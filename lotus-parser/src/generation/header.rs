use super::{HEADER_MEMORY_WASM_PAGE_COUNT, ToWat, ToWatVec, VALUE_BYTE_SIZE, Wat, wat};

type Import = (&'static str, &'static str, &'static str, &'static[&'static str], Option<&'static str>);
type Memory = (Option<&'static str>, usize);
type Global = (&'static str, &'static str);
type Function = (&'static str, &'static[(&'static str, &'static str)], Option<&'static str>, &'static[(&'static str, &'static str)], fn() -> Vec<Wat>);

pub const LOG_BOOL_FUNC_NAME : &'static str = "__log_bool";
pub const LOG_INT_FUNC_NAME : &'static str = "__log_int";
pub const LOG_FLOAT_FUNC_NAME : &'static str = "__log_float";
pub const LOG_STRING_FUNC_NAME : &'static str = "__log_string";

// get stack order: self, index
pub const DEREF_INT_POINTER_GET_FUNC_NAME : &'static str = "__ptr_get_i32";
pub const DEREF_FLOAT_POINTER_GET_FUNC_NAME : &'static str = "__ptr_get_f32";
// set order: value, self, index
pub const DEREF_INT_POINTER_SET_FUNC_NAME : &'static str = "__ptr_set_i32";
pub const DEREF_FLOAT_POINTER_SET_FUNC_NAME : &'static str = "__ptr_set_f32";

pub const HEADER_IMPORTS : &'static[Import] = &[
    ("log", "bool", LOG_BOOL_FUNC_NAME, &["i32"], None),
    ("log", "int", LOG_INT_FUNC_NAME, &["i32"], None),
    ("log", "float", LOG_FLOAT_FUNC_NAME, &["f32"], None),
    ("log", "string", LOG_STRING_FUNC_NAME, &["i32"], None),
];

pub const HEADER_MEMORIES : &'static[Memory] = &[
    (Some("memory"), HEADER_MEMORY_WASM_PAGE_COUNT)
];

pub const HEADER_GLOBALS : &'static[Global] = &[
];

pub static HEADER_FUNCTIONS : &'static[Function] = &[
    (DEREF_INT_POINTER_GET_FUNC_NAME, &[("pointer", "i32"), ("index", "i32")], Some("i32"), &[], ptr_get_int),
    (DEREF_INT_POINTER_SET_FUNC_NAME, &[("value", "i32"), ("pointer", "i32"), ("index", "i32")], None, &[], ptr_set_int)
];

fn ptr_get_int() -> Vec<Wat> {
    vec![
        wat!["i32.mul", wat!["i32.add", Wat::get_local("pointer"), Wat::get_local("index")], Wat::const_i32(VALUE_BYTE_SIZE)],
        wat!["i32.load"],
    ]
}

fn ptr_set_int() -> Vec<Wat> {
    vec![
        wat!["i32.mul", wat!["i32.add", Wat::get_local("pointer"), Wat::get_local("index")], Wat::const_i32(VALUE_BYTE_SIZE)],
        wat!["i32.store", Wat::get_local("value")]
    ]
}