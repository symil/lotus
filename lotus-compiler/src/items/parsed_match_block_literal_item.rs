use parsable::{parsable, DataLocation};
use crate::program::{Vasm, ProgramContext, EQ_METHOD_NAME, Type};
use super::{ParsedNoneLiteral, ParsedNumberLiteral, ParsedStringLiteral, ParsedCharLiteral, ParsedBooleanLiteral};

#[parsable]
pub enum ParsedMatchBlockLiteralItem {
    None(ParsedNoneLiteral),
    Boolean(ParsedBooleanLiteral),
    Number(ParsedNumberLiteral),
    String(ParsedStringLiteral),
    Character(ParsedCharLiteral),
}

impl ParsedMatchBlockLiteralItem {
    fn get_location(&self) -> &DataLocation {
        match self {
            ParsedMatchBlockLiteralItem::None(value) => value,
            ParsedMatchBlockLiteralItem::Boolean(value) => value,
            ParsedMatchBlockLiteralItem::Number(value) => value,
            ParsedMatchBlockLiteralItem::String(value) => value,
            ParsedMatchBlockLiteralItem::Character(value) => value,
        }
    }

    pub fn process(&self, tested_value: Vasm, context: &mut ProgramContext) -> Option<(Type, Vasm)> {
        let type_hint = Some(&tested_value.ty);
        let item_vasm = match self {
            ParsedMatchBlockLiteralItem::None(none_literal) => none_literal.process(type_hint, context),
            ParsedMatchBlockLiteralItem::Boolean(bool_literal) => bool_literal.process(context),
            ParsedMatchBlockLiteralItem::Number(number_literal) => number_literal.process(type_hint, context),
            ParsedMatchBlockLiteralItem::String(string_literal) => string_literal.process(context),
            ParsedMatchBlockLiteralItem::Character(char_literal) => char_literal.process(context),
        }?;

        if !item_vasm.ty.is_assignable_to(&tested_value.ty) {
            context.errors.type_mismatch(self.get_location(), &tested_value.ty, &item_vasm.ty);
            return None;
        }

        let item_type = item_vasm.ty.clone();

        Some((
            item_type.clone(),
            context.vasm()
                .append(tested_value)
                .call_regular_method(&item_type, EQ_METHOD_NAME, &[], vec![item_vasm], context)
                .set_type(context.bool_type())
        ))
    }
}