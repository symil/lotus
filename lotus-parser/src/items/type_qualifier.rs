use parsable::parsable;

#[parsable(impl_display=true)]
#[derive(PartialEq, Copy)]
pub enum TypeQualifier {
    Type = "type",
    Struct = "struct",
    View = "view",
    Entity = "entity",
    Event = "event",
    World = "world",
    User = "user",
    Request = "request"
}

impl TypeQualifier {
    pub fn is_entity_qualifier(&self) -> bool {
        match self {
            TypeQualifier::Type => false,
            TypeQualifier::Struct => false,
            TypeQualifier::View => false,
            TypeQualifier::Entity => true,
            TypeQualifier::Event => false,
            TypeQualifier::World => true,
            TypeQualifier::User => true,
            TypeQualifier::Request => false,
        }
    }
}

impl Default for TypeQualifier {
    fn default() -> Self {
        Self::Struct
    }
}