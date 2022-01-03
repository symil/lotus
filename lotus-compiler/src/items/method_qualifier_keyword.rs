use parsable::parsable;
use crate::program::{FieldKind, MethodQualifier};

#[parsable]
pub struct MethodQualifierKeyword {
    pub value: MethodQualifierKeywordValue
}

#[parsable]
#[derive(Clone, Copy, PartialEq)]
pub enum MethodQualifierKeywordValue {
    Static = "static",
    Dyn = "dyn"
}

impl MethodQualifierKeyword {
    pub fn process(&self) -> MethodQualifier {
        match &self.value {
            MethodQualifierKeywordValue::Static => MethodQualifier::Static,
            MethodQualifierKeywordValue::Dyn => MethodQualifier::Dynamic,
        }
    }
}