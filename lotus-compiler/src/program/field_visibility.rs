use super::{SYSTEM_FIELD_PREFIX, PRIVATE_FIELD_PREFIX};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum FieldVisibility {
    Public,
    Private,
    System
}

impl FieldVisibility {
    pub fn from_name(name: &str) -> Self {
        if name.starts_with(SYSTEM_FIELD_PREFIX) {
            Self::System
        } else if name.starts_with(PRIVATE_FIELD_PREFIX) {
            Self::Private
        } else {
            Self::Public
        }
    }

    pub fn is_private(&self) -> bool {
        self == &FieldVisibility::Private
    }

    pub fn is_public(&self) -> bool {
        self == &FieldVisibility::Public
    }
}