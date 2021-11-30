use crate::{program::MEMORY_CELL_BYTE_SIZE, wat};
use super::{HEADER_MEMORY_WASM_PAGE_COUNT, Wat};

pub const DUMMY_FUNC_NAME : &'static str = "dummy";
pub const DUPLICATE_INT_WASM_FUNC_NAME : &'static str = "dup_i32";
pub const SWAP_INT_INT_WASM_FUNC_NAME : &'static str = "swap_i32_i32";
pub const SWAP_FLOAT_INT_WASM_FUNC_NAME : &'static str = "swap_f32_i32";
pub const LOAD_INT_WASM_FUNC_NAME : &'static str = "load_int";
pub const STORE_INT_WASM_FUNC_NAME : &'static str = "store_int";
pub const LOAD_FLOAT_WASM_FUNC_NAME : &'static str = "load_float";
pub const STORE_FLOAT_WASM_FUNC_NAME : &'static str = "store_float";

pub const TMP_VAR_NAME : &'static str = "tmp";
pub const CLOSURE_TMP_VAR_NAME : &'static str = "tmp_closure";
pub const CLOSURE_VARIABLES_TMP_VAR_NAME : &'static str = "tmp_closure_variables";
pub const CLOSURE_ARGS_TYPE_NAME : &'static str = "retain_closure_args_func_type";

type Import = (&'static str, &'static str, &'static str, &'static[&'static str], Option<&'static str>);
type Memory = (Option<&'static str>, usize);
type Table = (usize, &'static str);
type FunctionType = (&'static str, &'static[&'static str], &'static[&'static str]);
type Global = (&'static str, &'static str);
type Function = (&'static str, &'static[(&'static str, &'static str)], &'static[&'static str], &'static[(&'static str, &'static str)], fn() -> Vec<Wat>);

pub const HEADER_IMPORTS : &'static[Import] = &[
    ("utils", "assert", "assert", &["i32", "i32"], None),
    ("utils", "float_to_string", "float_to_string", &["f32", "i32"], None),
    ("env", "log", "log", &["i32"], None),
    ("env", "log_int", "log_int", &["i32"], None),
    ("env", "get_current_time", "get_current_time", &[], Some("i32")),
    ("client", "init_window", "init_window", &["f32"], None),
    ("client", "poll_events", "poll_events", &["i32", "i32"], Some("i32")),
    ("client", "get_window_width", "get_window_width", &[], Some("f32")),
    ("client", "get_window_height", "get_window_height", &[], Some("f32")),
    ("client", "draw_frame", "draw_frame", &["i32", "i32"], None),
    // ("client", "connect", "connect", &["i32"], Some("i32")),
    // ("client", "is_connected", "is_connected", &[], Some("i32")),
    // ("client", "read_message", "read_message", &["i32"], Some("i32")),
    // ("client", "send_message", "send_message", &["i32", "i32"], None),
];

pub const HEADER_MEMORIES : &'static[Memory] = &[
    (Some("memory"), HEADER_MEMORY_WASM_PAGE_COUNT)
];

pub const HEADER_FUNC_TYPES : &'static[FunctionType] = &[
    (CLOSURE_ARGS_TYPE_NAME, &["i32"], &[])
];

pub const HEADER_GLOBALS : &'static[Global] = &[
    (TMP_VAR_NAME, "i32"),
    (CLOSURE_TMP_VAR_NAME, "i32"),
    (CLOSURE_VARIABLES_TMP_VAR_NAME, "i32")
];

pub static HEADER_FUNCTIONS : &'static[Function] = &[
    (DUMMY_FUNC_NAME, &[], &[], &[], dummy),
    (DUPLICATE_INT_WASM_FUNC_NAME, &[("arg", "i32")], &["i32", "i32"], &[], duplicate_value),
    (SWAP_INT_INT_WASM_FUNC_NAME, &[("arg1", "i32"), ("arg2", "i32")], &["i32", "i32"], &[], swap_values),
    (SWAP_FLOAT_INT_WASM_FUNC_NAME, &[("arg1", "f32"), ("arg2", "i32")], &["i32", "f32"], &[], swap_values),
    (LOAD_INT_WASM_FUNC_NAME, &[("addr", "i32")], &["i32"], &[], load_int),
    (STORE_INT_WASM_FUNC_NAME, &[("addr", "i32"), ("value", "i32")], &[], &[], store_int),
    (LOAD_FLOAT_WASM_FUNC_NAME, &[("addr", "i32")], &["f32"], &[], load_float),
    (STORE_FLOAT_WASM_FUNC_NAME, &[("addr", "i32"), ("value", "f32")], &[], &[], store_float),
];

fn dummy() -> Vec<Wat> {
    vec![]
}

fn duplicate_value() -> Vec<Wat> {
    vec![
        Wat::get_local("arg"),
        Wat::get_local("arg"),
    ]
}

fn swap_values() -> Vec<Wat> {
    vec![
        Wat::get_local("arg2"),
        Wat::get_local("arg1"),
    ]
}

fn load_int() -> Vec<Wat> {
    vec![
        Wat::get_local("addr"),
        wat!["i32.mul", Wat::const_i32(4)],
        wat!["i32.load"]
    ]
}

fn store_int() -> Vec<Wat> {
    vec![
        Wat::get_local("addr"),
        wat!["i32.mul", Wat::const_i32(4)],
        Wat::get_local("value"),
        wat!["i32.store"]
    ]
}

fn load_float() -> Vec<Wat> {
    vec![
        Wat::get_local("addr"),
        wat!["i32.mul", Wat::const_i32(4)],
        wat!["f32.load"]
    ]
}

fn store_float() -> Vec<Wat> {
    vec![
        Wat::get_local("addr"),
        wat!["i32.mul", Wat::const_i32(4)],
        Wat::get_local("value"),
        wat!["f32.store"]
    ]
}