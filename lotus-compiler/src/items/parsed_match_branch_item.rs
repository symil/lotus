use parsable::{parsable, ItemLocation};
use crate::program::{Type, ProgramContext, Vasm};
use super::{ParsedNoneLiteral, ParsedNumberLiteral, ParsedType, ParsedDoubleColon, Identifier, ParsedMatchBranchTypeItem, ParsedMatchBranchLiteralItem, ParsedWildcard};

#[parsable]
pub enum ParsedMatchBranchItem {
    Wildcard(ParsedWildcard),
    Literal(ParsedMatchBranchLiteralItem),
    TypeOrEnumVariant(ParsedMatchBranchTypeItem),
}

impl ParsedMatchBranchItem {
    pub fn get_location(&self) -> &ItemLocation {
        match self {
            ParsedMatchBranchItem::Wildcard(value) => value,
            ParsedMatchBranchItem::Literal(value) => value.get_location(),
            ParsedMatchBranchItem::TypeOrEnumVariant(value) => value,
        }
    }

    pub fn get_variant_name(&self) -> Option<String> {
        match self {
            ParsedMatchBranchItem::Wildcard(_) => None,
            ParsedMatchBranchItem::Literal(literal) => match literal {
                ParsedMatchBranchLiteralItem::None(_) => None,
                ParsedMatchBranchLiteralItem::Boolean(bool_literal) => Some(bool_literal.token.as_str().to_string()),
                ParsedMatchBranchLiteralItem::Number(_) => None,
                ParsedMatchBranchLiteralItem::String(_) => None,
                ParsedMatchBranchLiteralItem::Character(_) => None,
            },
            ParsedMatchBranchItem::TypeOrEnumVariant(item) => match &item.variant {
                Some(variant) => item.ty.as_single_identifier()
                    .zip(variant.name.as_ref())
                    .map(|(type_name, variant_name)| {
                        format!("{}::{}", type_name.as_str(), variant_name.as_str())
                    }
                ),
                None => None,
            }
        }
    }

    pub fn process(&self, tested_value: Vasm, context: &mut ProgramContext) -> Option<(Type, Vasm)> {
        match self {
            ParsedMatchBranchItem::Wildcard(_) => Some((tested_value.ty.clone(), context.vasm().int(1i32).set_type(context.bool_type()))),
            ParsedMatchBranchItem::Literal(literal) => literal.process(tested_value, context),
            ParsedMatchBranchItem::TypeOrEnumVariant(type_or_enum_variant) => type_or_enum_variant.process(tested_value, context),
        }
    }
}