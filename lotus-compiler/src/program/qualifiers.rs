use super::FieldKind;

// TODO: split this file into multiple files

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Visibility {
    None,
    Private,
    Public,
    Export,
    System,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EventCallbackQualifier {
    Standard,
    TargetSelf
}

impl EventCallbackQualifier {
    pub fn get_default_priority(&self) -> i32 {
        match self {
            Self::Standard => 0,
            Self::TargetSelf => 0,
        }
    }

    pub fn get_event_field_name(&self) -> Option<&'static str> {
        match self {
            Self::Standard => None,
            Self::TargetSelf => Some("target"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MethodMetaQualifier {
    None,
    Autogen
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MethodQualifier {
    None,
    Static,
    Dynamic
}

impl MethodQualifier {
    pub fn to_field_kind(self) -> FieldKind {
        match self {
            Self::None => FieldKind::Regular,
            Self::Static => FieldKind::Static,
            Self::Dynamic => FieldKind::Regular,
        }
    }
}