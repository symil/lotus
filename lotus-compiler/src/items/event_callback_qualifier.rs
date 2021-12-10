use parsable::parsable;

#[parsable]
#[derive(Clone)]
pub struct EventCallbackQualifier {
    value: EventCallbackQualifierValue
}

#[parsable]
#[derive(PartialEq, Hash, Eq, Clone, Copy)]
pub enum EventCallbackQualifierValue {
    Standard = "@",
    TargetSelf = "$",
}

impl EventCallbackQualifier {
    pub fn get_default_priority(&self) -> i32 {
        match &self.value {
            EventCallbackQualifierValue::Standard => 0,
            EventCallbackQualifierValue::TargetSelf => 0,
        }
    }

    pub fn get_event_field_name(&self) -> Option<&'static str> {
        match &self.value {
            EventCallbackQualifierValue::Standard => None,
            EventCallbackQualifierValue::TargetSelf => Some("target"),
        }
    }
}