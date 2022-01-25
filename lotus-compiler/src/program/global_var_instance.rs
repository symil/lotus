use std::rc::Rc;
use crate::{items::ParsedVisibilityToken};
use super::{VariableInfo, Wat};

#[derive(Debug)]
pub struct GlobalVarInstance {
    pub wasm_name: String,
    pub wasm_type: &'static str,
    pub init_wat: Vec<Wat>,
    pub retain_wat: Vec<Wat>,
    pub wasm_locals: Vec<(&'static str, String)>,
    // Indicates if the variable should be initialized before anything else (e.g constants used during memory allocation)
    pub is_essential: bool,
}