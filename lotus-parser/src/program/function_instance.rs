use super::Wat;

#[derive(Debug)]
pub struct FunctionInstance {
    pub wasm_name: String,
    pub wasm_declaration: Option<Wat>,
    pub wasm_call: Vec<Wat>
}