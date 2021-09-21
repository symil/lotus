use std::hash::Hash;

use super::Wat;

#[derive(Debug)]
pub struct FunctionInstanceHeader {
    pub id: u64,
    pub wasm_name: String,
    pub wasm_call: Vec<Wat>
}

impl Hash for FunctionInstanceHeader {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}