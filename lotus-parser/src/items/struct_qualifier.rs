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

impl StructQualifier {
    pub fn is_entity_qualifier(&self) -> bool {
        match self {
            StructQualifier::Struct => false,
            StructQualifier::View => false,
            StructQualifier::Entity => true,
            StructQualifier::Event => false,
            StructQualifier::World => true,
            StructQualifier::User => true,
            StructQualifier::Request => false,
        }
    }
}

impl Default for StructQualifier {
    fn default() -> Self {
        Self::Struct
    }
}