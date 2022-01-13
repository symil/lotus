use parsable::parsable;
use crate::program::{Type, ProgramContext, Vasm};
use super::{ParsedNoneLiteral, ParsedNumberLiteral, ParsedType, ParsedDoubleColon, Identifier, ParsedMatchBlockTypeItem, ParsedMatchBlockLiteralItem, ParsedWildcard};

#[parsable]
pub enum ParsedMatchBlockItem {
    Wildcard(ParsedWildcard),
    Literal(ParsedMatchBlockLiteralItem),
    TypeOrEnumVariant(ParsedMatchBlockTypeItem),
}

impl ParsedMatchBlockItem {
    pub fn process(&self, tested_value: Vasm, context: &mut ProgramContext) -> Option<(Type, Vasm)> {
        match self {
            ParsedMatchBlockItem::Wildcard(_) => Some((tested_value.ty.clone(), context.vasm().int(1i32).set_type(context.bool_type()))),
            ParsedMatchBlockItem::Literal(literal) => literal.process(tested_value, context),
            ParsedMatchBlockItem::TypeOrEnumVariant(type_or_enum_variant) => type_or_enum_variant.process(tested_value, context),
        }
    }
}