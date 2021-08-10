use crate::generation::Wat;
use super::Type;

#[derive(Default)]
pub struct GlobalAnnotation {
    pub wasm_name: String,
    pub ty: Type,
    pub value: Wat
}