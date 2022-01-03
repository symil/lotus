use parsable::parsable;

use crate::program::MethodMetaQualifier;

#[parsable]
pub struct MethodMetaQualifierKeyword {
    pub value: MethodMetaQualifierKeywordValue
}

#[parsable]
#[derive(Clone, Copy, PartialEq)]
pub enum MethodMetaQualifierKeywordValue {
    Autogen = "autogen"
}

impl MethodMetaQualifierKeyword {
    pub fn process(&self) -> MethodMetaQualifier {
        match &self.value {
            MethodMetaQualifierKeywordValue::Autogen => MethodMetaQualifier::Autogen,
        }
    }
}