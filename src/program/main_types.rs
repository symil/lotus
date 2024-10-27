use enum_iterator::Sequence;
use super::BuiltinType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Sequence)]
pub enum MainType {
    World,
    User,
    Window,
    LocalData,
    GameInstance,
}