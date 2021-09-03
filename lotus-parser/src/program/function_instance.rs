use crate::generation::Wat;

pub struct FunctionInstance {
    pub id: u64,
    pub wasm_name: String,
    pub wasm_declaration: Wat
}