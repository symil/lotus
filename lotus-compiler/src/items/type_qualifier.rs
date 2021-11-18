use parsable::parsable;
use crate::program::TypeCategory;

#[parsable(impl_display=true)]
#[derive(PartialEq, Clone, Copy)]
pub enum TypeQualifier {
    Type = "type",
    Enum = "enum",
    Class = "class",
    View = "view"
}

impl TypeQualifier {
    pub fn to_type_category(&self) -> TypeCategory {
        match self {
            TypeQualifier::Type => TypeCategory::Type,
            TypeQualifier::Enum => TypeCategory::Enum,
            TypeQualifier::Class | TypeQualifier::View => TypeCategory::Class,
        }
    }
}

impl Default for TypeQualifier {
    fn default() -> Self {
        Self::Type
    }
}