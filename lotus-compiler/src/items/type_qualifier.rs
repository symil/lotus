use parsable::parsable;
use crate::program::TypeCategory;

#[parsable(impl_display=true)]
#[derive(PartialEq, Clone, Copy)]
pub enum TypeQualifier {
    Type = "type",
    Enum = "enum",
    Class = "class",
}

impl TypeQualifier {
    pub fn to_type_category(&self) -> TypeCategory {
        match self {
            TypeQualifier::Type => TypeCategory::Type,
            TypeQualifier::Enum => TypeCategory::Enum,
            TypeQualifier::Class => TypeCategory::Class,
        }
    }
}

impl Default for TypeQualifier {
    fn default() -> Self {
        Self::Type
    }
}