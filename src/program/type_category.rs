use super::WasmStackType;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TypeCategory {
    Type,
    Enum,
    Class
}

impl TypeCategory {
    pub fn get_default_wasm_stack_type(&self) -> WasmStackType {
        match self {
            TypeCategory::Type => WasmStackType::I32,
            TypeCategory::Enum => WasmStackType::I32,
            TypeCategory::Class => WasmStackType::I32,
        }
    }
}