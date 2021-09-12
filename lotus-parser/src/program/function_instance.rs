use crate::generation::Wat;

pub struct FunctionInstance {
    pub wasm_name: String,
    pub wasm_declaration: Option<Wat>,
    pub wasm_call: Vec<Wat>
}