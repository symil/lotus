use parsable::parsable;

#[parsable(impl_display=true)]
#[derive(PartialEq, Copy)]
pub enum StructQualifier {
    Struct = "struct",
    View = "view",
    Entity = "entity",
    Event = "event",
    World = "world",
    User = "user",
    Request = "request"
}

impl Default for StructQualifier {
    fn default() -> Self {
        Self::Struct
    }
}