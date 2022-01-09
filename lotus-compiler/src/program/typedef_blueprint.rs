use crate::items::{Identifier, ParsedVisibilityToken};
use super::{GlobalItem, ProgramContext, Type, Visibility};

#[derive(Debug)]
pub struct TypedefBlueprint {
    pub type_id: u64,
    pub name: Identifier,
    pub visibility: Visibility,
    pub target: Type
}

impl TypedefBlueprint {
    pub fn check_types_parameters(&self, context: &mut ProgramContext) {
        self.target.check_parameters(context);
    }
}

impl GlobalItem for TypedefBlueprint {
    fn get_name(&self) -> &Identifier { &self.name }
    fn get_visibility(&self) -> Visibility { self.visibility }
}