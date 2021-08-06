use crate::{merge};
use super::{MainFunction, Memory, StdLib, ToWatVec, Wat};

pub struct WasmModule {
    pub std: StdLib,
    pub memory: Memory,
    pub main_function: MainFunction
}

impl WasmModule {
    pub fn new() -> Self {
        Self {
            std: StdLib::new(),
            memory: Memory::new(),
            main_function: MainFunction::new()
        }
    }

    pub fn generate_wat(&self) -> String {
        Wat::new("module", merge![
            self.std.get_header(),
            self.memory.get_header(),

            self.std.get_functions(),
            self.memory.get_functions(self),
            self.main_function.get_functions(self)
        ]).to_string(0)
    }
}

impl Default for WasmModule {
    fn default() -> Self {
        Self::new()
    }
}