use enum_iterator::IntoEnumIterator;
use super::BuiltinType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, IntoEnumIterator)]
pub enum MainType {
    World,
    User,
    Window,
    LocalData,
    GameInstance,
}