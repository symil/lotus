use parsable::{parsable, DataLocation};
use crate::program::{Type, ProgramContext, Vasm};
use super::{ParsedNoneLiteral, ParsedNumberLiteral, ParsedType, ParsedDoubleColon, Identifier, ParsedMatchBranchTypeItem, ParsedMatchBranchLiteralItem, ParsedWildcard};

#[parsable]
pub enum ParsedMatchBranchItem {
    Wildcard(ParsedWildcard),
    Literal(ParsedMatchBranchLiteralItem),
    TypeOrEnumVariant(ParsedMatchBranchTypeItem),
}

impl ParsedMatchBranchItem {
    pub fn get_location(&self) -> &DataLocation {
        match self {
            ParsedMatchBranchItem::Wildcard(value) => value,
            ParsedMatchBranchItem::Literal(value) => value.get_location(),
            ParsedMatchBranchItem::TypeOrEnumVariant(value) => value,
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