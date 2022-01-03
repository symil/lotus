use parsable::parsable;

use crate::program::EventCallbackQualifier;

#[parsable]
#[derive(Clone)]
pub struct EventCallbackQualifierKeyword {
    pub value: EventCallbackQualifierKeywordValue
}

#[parsable]
#[derive(PartialEq, Hash, Eq, Clone, Copy)]
pub enum EventCallbackQualifierKeywordValue {
    Standard = "@",
    TargetSelf = "$",
}

impl EventCallbackQualifierKeyword {
    pub fn process(&self) -> EventCallbackQualifier {
        match &self.value {
            EventCallbackQualifierKeywordValue::Standard => EventCallbackQualifier::Standard,
            EventCallbackQualifierKeywordValue::TargetSelf => EventCallbackQualifier::TargetSelf,
        }
    }
}