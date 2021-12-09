use parsable::parsable;

#[parsable]
#[derive(PartialEq, Hash, Eq, Clone, Copy)]
pub enum EventCallbackQualifier {
    Standard = "@",
    TargetSelf = "$",
}

impl EventCallbackQualifier {
    pub fn get_default_priority(&self) -> i32 {
        match self {
            EventCallbackQualifier::Standard => 1,
            EventCallbackQualifier::TargetSelf => 0,
        }
    }
}