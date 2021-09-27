use parsable::parsable;

use crate::program::FieldKind;

#[parsable]
#[derive(Clone, Copy, PartialEq)]
pub enum MethodQualifier {
    Regular,
    Static = "static",
    Dynamic = "dyn"
}

impl MethodQualifier {
    pub fn to_field_kind(self) -> FieldKind {
        match self {
            MethodQualifier::Regular => FieldKind::Regular,
            MethodQualifier::Static => FieldKind::Static,
            MethodQualifier::Dynamic => FieldKind::Regular,
        }
    }
}