use crate::items::Identifier;
use super::{Vasm, Type};

#[derive(Debug, Clone)]
pub struct ArgumentInfo {
    pub name: Identifier,
    pub ty: Type,
    pub is_optional: bool,
    pub default_value: Vasm
}