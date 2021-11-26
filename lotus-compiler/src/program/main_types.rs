use enum_iterator::IntoEnumIterator;

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

    pub fn get_default_name(&self) -> &'static str {
        match self {
            MainType::World => "BaseWorld",
            MainType::User => "BaseUser",
            MainType::Window => "View",
            MainType::LocalData => "Object",
        }
    }
}