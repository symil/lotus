use crate::utils::Link;
use super::{FunctionBlueprint, Type};

#[derive(Debug, Clone)]
pub struct FuncRef {
    pub function: Link<FunctionBlueprint>,
    pub this_type: Type
}