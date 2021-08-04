use super::WasmModule;

pub fn generate_wat() -> String {
    WasmModule::new().generate_wat()
}