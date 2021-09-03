use parsable::DataLocation;
use super::{GlobalItem, ItemVisibility};

#[derive(Debug)]
pub struct GlobalVarBlueprint {
    pub id: u64,
    pub name: String,
    pub location: DataLocation,
    pub visibility: ItemVisibility,
}

impl GlobalItem for GlobalVarBlueprint {
    fn get_id(&self) -> u64 { self.id }
    fn get_name(&self) -> &str { &self.name }
    fn get_location(&self) -> &DataLocation { &self.location }
    fn get_visibility(&self) -> ItemVisibility { self.visibility }
}