use enum_iterator::IntoEnumIterator;
use super::BuiltinType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, IntoEnumIterator)]
pub enum MainType {
    World,
    User,
    Window,
    LocalData
}

impl MainType {
    pub fn get_name(&self) -> &'static str {
        match self {
            MainType::World => "World",
            MainType::User => "User",
            MainType::Window => "Window",
            MainType::LocalData => "LocalData",
        }
    }

    pub fn get_default_type(&self) -> BuiltinType {
        match self {
            MainType::World => BuiltinType::Object,
            MainType::User => BuiltinType::Object,
            MainType::Window => BuiltinType::View,
            MainType::LocalData => BuiltinType::Object,
        }
    }
}