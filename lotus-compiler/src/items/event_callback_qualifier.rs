use parsable::parsable;

#[parsable]
#[derive(PartialEq, Hash, Eq, Clone, Copy)]
pub enum EventCallbackQualifier {
    Hook = "^",
    Before = "&",
    After= "@",
}