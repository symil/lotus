use crate::{items::StackType, utils::Link};
use super::TypeBlueprint;

#[derive(Debug, Clone)]
pub struct ResolvedType {
    pub type_wrapped: Link<TypeBlueprint>,
    pub parameters: Vec<ResolvedType>
}

impl ResolvedType {
    pub fn get_wasm_type(&self) -> Option<&'static str> {
        match self.type_wrapped.borrow().stack_type {
            StackType::Void => None,
            StackType::Int => Some("i32"),
            StackType::Float => Some("f32"),
            StackType::Pointer => Some("i32"),
        }
    }
}