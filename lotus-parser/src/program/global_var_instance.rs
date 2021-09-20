use std::rc::Rc;
use crate::{items::Visibility};
use super::{VariableInfo, Wat};

#[derive(Debug)]
pub struct GlobalVarInstance {
    pub wasm_name: String,
    pub wasm_type: &'static str,
    pub init_value: Vec<Wat>
}