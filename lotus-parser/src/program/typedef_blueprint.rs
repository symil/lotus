use crate::items::{Identifier, Visibility};
use super::{GlobalItem, Type};

#[derive(Debug)]
pub struct TypedefBlueprint {
    pub type_id: u64,
    pub name: Identifier,
    pub visibility: Visibility,
    pub target: Type
}

impl GlobalItem for TypedefBlueprint {
    fn get_name(&self) -> &Identifier { &self.name }
    fn get_visibility(&self) -> Visibility { self.visibility }
}