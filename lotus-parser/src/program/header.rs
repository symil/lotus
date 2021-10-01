use crate::{program::VALUE_BYTE_SIZE, wat};
use super::{HEADER_MEMORY_WASM_PAGE_COUNT, SWAP_FLOAT_INT_WASM_FUNC_NAME, SWAP_INT_INT_WASM_FUNC_NAME, Wat};

type Import = (&'static str, &'static str, &'static str, &'static[&'static str], Option<&'static str>);
type Memory = (Option<&'static str>, usize);
type Table = (usize, &'static str);
type FunctionType = (&'static str, &'static[&'static str], Option<&'static str>);
type Global = (&'static str, &'static str);
type Function = (&'static str, &'static[(&'static str, &'static str)], &'static[&'static str], &'static[(&'static str, &'static str)], fn() -> Vec<Wat>);

pub const RETAIN_FUNC_TYPE_NAME : &'static str = "_type_func_retain";

pub const LOG_EMPTY_FUNC_NAME : &'static str = "__log_empty";
pub const LOG_BOOL_FUNC_NAME : &'static str = "__log_bool";
pub const LOG_INT_FUNC_NAME : &'static str = "__log_int";
pub const LOG_FLOAT_FUNC_NAME : &'static str = "__log_float";
pub const LOG_STRING_FUNC_NAME : &'static str = "__log_string";

pub const HEADER_IMPORTS : &'static[Import] = &[
    ("log", "empty", LOG_EMPTY_FUNC_NAME, &[], None),
    ("log", "bool", LOG_BOOL_FUNC_NAME, &["i32"], None),
    ("log", "int", LOG_INT_FUNC_NAME, &["i32"], None),
    ("log", "float", LOG_FLOAT_FUNC_NAME, &["f32"], None),
    ("log", "string", LOG_STRING_FUNC_NAME, &["i32"], None),
];

pub const HEADER_MEMORIES : &'static[Memory] = &[
    (Some("memory"), HEADER_MEMORY_WASM_PAGE_COUNT)
];

pub const HEADER_FUNC_TYPES : &'static[FunctionType] = &[
    (RETAIN_FUNC_TYPE_NAME, &["i32"], None)
];

pub const HEADER_GLOBALS : &'static[Global] = &[
];

pub static HEADER_FUNCTIONS : &'static[Function] = &[
    (SWAP_INT_INT_WASM_FUNC_NAME, &[("arg1", "i32"), ("arg2", "i32")], &["i32", "i32"], &[], swap_values),
    (SWAP_FLOAT_INT_WASM_FUNC_NAME, &[("arg1", "f32"), ("arg2", "i32")], &["i32", "f32"], &[], swap_values),
];

fn swap_values() -> Vec<Wat> {
    vec![
        Wat::get_local("arg2"),
        Wat::get_local("arg1"),
    ]
}