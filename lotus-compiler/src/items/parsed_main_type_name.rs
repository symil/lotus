use parsable::parsable;
use crate::program::MainType;

#[parsable(name = "main type name")]
pub enum ParsedMainTypeName {
    User = "USER_TYPE",
    World = "WORLD_TYPE",
    Window = "WINDOW_TYPE",
    LocalData = "LOCAL_DATA_TYPE",
}

impl ParsedMainTypeName {
    pub fn to_main_type(&self) -> MainType {
        match self {
            ParsedMainTypeName::User => MainType::User,
            ParsedMainTypeName::World => MainType::World,
            ParsedMainTypeName::Window => MainType::Window,
            ParsedMainTypeName::LocalData => MainType::LocalData,
        }
    }
}