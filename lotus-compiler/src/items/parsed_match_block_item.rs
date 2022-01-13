use parsable::parsable;
use crate::program::{Type, ProgramContext, Vasm};
use super::{ParsedNoneLiteral, ParsedNumberLiteral, ParsedType, ParsedDoubleColon, Identifier, ParsedMatchBlockTypeItem, ParsedMatchBlockLiteralItem};

#[parsable]
pub enum ParsedMatchBlockItem {
    Literal(ParsedMatchBlockLiteralItem),
    TypeOrEnumVariant(ParsedMatchBlockTypeItem),
}

impl ParsedMatchBlockItem {
    pub fn process(&self, tested_value: Vasm, context: &mut ProgramContext) -> Option<Vasm> {
        match self {
            ParsedMatchBlockItem::Literal(literal) => literal.process(tested_value, context),
            ParsedMatchBlockItem::TypeOrEnumVariant(type_or_enum_variant) => type_or_enum_variant.process(tested_value, context),
        }
    }
}