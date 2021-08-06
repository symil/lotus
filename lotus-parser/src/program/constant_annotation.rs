use crate::generation::Wat;
use super::Type;

pub struct ConstantAnnotation {
    expr_type: Type,
    value: Wat
}