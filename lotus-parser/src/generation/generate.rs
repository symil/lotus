use crate::{generation::{Imports, MainFunction, Memory, Wat, WasmModule}, merge};

pub fn generate_wat() -> String {
    WasmModule::new().generate_wat()
}