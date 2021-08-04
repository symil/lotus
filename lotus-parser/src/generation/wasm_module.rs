use crate::{merge};
use super::{Imports, MainFunction, Memory, Wat, ToWatVec};

pub struct WasmModule {
    pub imports: Imports,
    pub memory: Memory,
    pub main_function: MainFunction
}

impl WasmModule {
    pub fn new() -> Self {
        Self {
            imports: Imports::new(),
            memory: Memory::new(),
            main_function: MainFunction::new()
        }
    }

    pub fn generate_wat(&self) -> String {
        Wat::new("module", merge![
            self.imports.get_header(),
            self.memory.get_header(),
            self.main_function.get_header(self)
        ]).to_string(0)
    }
}