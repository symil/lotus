use crate::{generation::Wat, items::VisibilityToken};
use super::{Type, VariableInfo};

#[derive(Debug)]
pub struct GlobalVarInstance {
    pub var_info: VariableInfo,
    pub value: Vec<Wat>
}