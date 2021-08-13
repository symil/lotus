use super::{LOG_I32_FUNC_NAME, DEREF_INT_POINTER_GET_FUNC_NAME, DEREF_INT_POINTER_SET_FUNC_NAME, ToWat, ToWatVec, VALUE_BYTE_SIZE, Wat, wat};

type Import = (&'static str, &'static str, &'static str, &'static[&'static str], Option<&'static str>);
type Memory = (Option<&'static str>, usize);
type Global = (&'static str, &'static str);
type Function = (&'static str, &'static[(&'static str, &'static str)], Option<&'static str>, &'static[(&'static str, &'static str)], fn() -> Vec<Wat>);

pub const HEADER_IMPORTS : &'static[Import] = &[
    ("log", "i32", LOG_I32_FUNC_NAME, &["i32"], None)
];

pub const HEADER_MEMORIES : &'static[Memory] = &[
    (Some("memory"), 1)
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