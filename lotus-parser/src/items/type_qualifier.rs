use parsable::parsable;

#[parsable(impl_display=true)]
#[derive(PartialEq, Clone, Copy)]
pub enum TypeQualifier {
    Type = "type",
    Class = "class",
    Enum = "enum"
}

impl TypeQualifier {
    pub fn is_entity_qualifier(&self) -> bool {
        todo!()
    }
}

impl Default for TypeQualifier {
    fn default() -> Self {
        Self::Type
    }
}