use crate::{generation::Wat, items::Visibility};
use super::{TypeOld, VariableInfo};

#[derive(Debug)]
pub struct GlobalVarInstance {
    pub var_info: VariableInfo,
    pub value: Vec<Wat>
}