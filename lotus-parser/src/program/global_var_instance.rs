use crate::{items::Visibility};
use super::{VariableInfo, Wat};

#[derive(Debug)]
pub struct GlobalVarInstance {
    pub var_info: VariableInfo,
    pub value: Vec<Wat>
}