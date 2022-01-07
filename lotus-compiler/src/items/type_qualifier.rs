use parsable::parsable;
use crate::program::{TypeCategory, BuiltinType};

#[parsable(impl_display=true)]
#[derive(PartialEq, Clone, Copy)]
pub enum TypeQualifier {
    Type = "type",
    Enum = "enum",
    Class = "class",
    View = "view",
    Event = "event"
}

impl TypeQualifier {
    pub fn to_type_category(&self) -> TypeCategory {
        match self {
            TypeQualifier::Type => TypeCategory::Type,
            TypeQualifier::Enum => TypeCategory::Enum,
            TypeQualifier::Class => TypeCategory::Class,
            TypeQualifier::View => TypeCategory::Class,
            TypeQualifier::Event => TypeCategory::Class,
        }
    }

    pub fn get_inherited_type(&self) -> Option<BuiltinType> {
        match self {
            TypeQualifier::Type => None,
            TypeQualifier::Enum => Some(BuiltinType::Enum),
            TypeQualifier::Class => Some(BuiltinType::Object),
            TypeQualifier::View => Some(BuiltinType::View),
            TypeQualifier::Event => Some(BuiltinType::Event),
        }
    }
}

impl Default for TypeQualifier {
    fn default() -> Self {
        Self::Type
    }
}