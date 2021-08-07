use crate::generation::Wat;
use super::Type;

pub struct ConstantAnnotation {
    wasm_name: String,
    expr_type: Type,
    value: Wat
}