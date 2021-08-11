use super::LOG_I32_FUNC_NAME;

type Import = (&'static str, &'static str, &'static str, &'static[&'static str], Option<&'static str>);

pub const IMPORT_LIST : &'static[Import] = &[
    ("log", "i32", LOG_I32_FUNC_NAME, &["i32"], None)
];