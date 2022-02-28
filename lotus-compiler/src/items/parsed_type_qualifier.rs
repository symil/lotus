use parsable::parsable;
use crate::program::{TypeCategory, BuiltinType, WasmStackType};

#[parsable(impl_display=true)]
#[derive(PartialEq, Clone, Copy)]
pub enum ParsedTypeQualifier {
    Type = "type",
    Enum = "enum",
    Class = "class",
    View = "view",
    Event = "event"
}

impl ParsedTypeQualifier {
    pub fn to_type_category(&self) -> TypeCategory {
        match self {
            ParsedTypeQualifier::Type => TypeCategory::Type,
            ParsedTypeQualifier::Enum => TypeCategory::Enum,
            ParsedTypeQualifier::Class => TypeCategory::Class,
            ParsedTypeQualifier::View => TypeCategory::Class,
            ParsedTypeQualifier::Event => TypeCategory::Class,
        }
    }

    pub fn get_inherited_type(&self) -> Option<BuiltinType> {
        match self {
            ParsedTypeQualifier::Type => Some(BuiltinType::Any),
            ParsedTypeQualifier::Enum => Some(BuiltinType::Enum),
            ParsedTypeQualifier::Class => Some(BuiltinType::Object),
            ParsedTypeQualifier::View => Some(BuiltinType::View),
            ParsedTypeQualifier::Event => Some(BuiltinType::Event),
        }
    }
}

impl Default for ParsedTypeQualifier {
    fn default() -> Self {
        Self::Type
    }
}