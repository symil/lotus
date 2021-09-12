use std::rc::Rc;
use parsable::DataLocation;
use crate::items::{Identifier, Visibility};
use super::{GlobalItem, VariableInfo, Vasm};

#[derive(Debug)]
pub struct GlobalVarBlueprint {
    pub var_id: u64,
    pub visibility: Visibility,
    pub var_info: Rc<VariableInfo>,
    pub init_vasm: Vasm
}

impl GlobalItem for GlobalVarBlueprint {
    fn get_name(&self) -> &Identifier { &self.var_info.name }
    fn get_visibility(&self) -> Visibility { self.visibility }
}