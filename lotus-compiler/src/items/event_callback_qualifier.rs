use parsable::parsable;

#[parsable]
#[derive(PartialEq)]
pub enum EventCallbackQualifier {
    Hook = "`",
    Before = "'",
    After= "\"",
}