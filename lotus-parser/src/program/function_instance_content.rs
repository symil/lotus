use crate::utils::Link;
use super::Wat;

#[derive(Debug)]
pub struct FunctionInstanceContent {
    pub wasm_declaration: Option<Wat>
}