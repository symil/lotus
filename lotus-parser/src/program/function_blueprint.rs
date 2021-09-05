use indexmap::IndexSet;
use parsable::DataLocation;
use crate::{generation::Wat};
use super::{FunctionInstance, GlobalItem, ProgramContext, Type, ItemVisibility, WasmBlueprint};

#[derive(Debug)]
pub struct FunctionBlueprint {
    pub id: u64,
    pub name: String,
    pub location: DataLocation,
    pub visibility: ItemVisibility,
    pub generics: IndexSet<String>,
    pub owner: Option<u64>,
    pub this_type: Option<Type>,
    pub payload_type: Option<Type>,
    pub arguments: Vec<(String, Type)>,
    pub return_type: Option<Type>,
    pub body: Vec<Wat>
}

impl FunctionBlueprint {
    pub fn to_instance(&self, generics: Vec<u64>, context: &mut ProgramContext) -> FunctionInstance {
        todo!()
    }

    pub fn is_static(&self) -> bool {
        self.this_type.is_none()
    }
}

impl GlobalItem for FunctionBlueprint {
    fn get_id(&self) -> u64 { self.id }
    fn get_name(&self) -> &str { &self.name }
    fn get_location(&self) -> &DataLocation { &self.location }
    fn get_visibility(&self) -> ItemVisibility { self.visibility }
}