use parsable::{parsable, ItemLocation};
use crate::program::{Vasm, ProgramContext, EQ_METHOD_NAME, Type};
use super::{ParsedNoneLiteral, ParsedNumberLiteral, ParsedStringLiteral, ParsedCharLiteral, ParsedBooleanLiteral};

#[parsable]
pub enum ParsedMatchBranchLiteralItem {
    None(ParsedNoneLiteral),
    Boolean(ParsedBooleanLiteral),
    Number(ParsedNumberLiteral),
    String(ParsedStringLiteral),
    Character(ParsedCharLiteral),
}

impl ParsedMatchBranchLiteralItem {
    pub fn process(&self, tested_value: Vasm, context: &mut ProgramContext) -> Option<(Type, Vasm)> {
        let type_hint = Some(&tested_value.ty);
        let item_vasm = match self {
            ParsedMatchBranchLiteralItem::None(none_literal) => none_literal.process(type_hint, context),
            ParsedMatchBranchLiteralItem::Boolean(bool_literal) => bool_literal.process(context),
            ParsedMatchBranchLiteralItem::Number(number_literal) => number_literal.process(type_hint, context),
            ParsedMatchBranchLiteralItem::String(string_literal) => string_literal.process(context),
            ParsedMatchBranchLiteralItem::Character(char_literal) => char_literal.process(context),
        }?;

        if !item_vasm.ty.is_assignable_to(&tested_value.ty) {
            context.errors.type_mismatch(self, &tested_value.ty, &item_vasm.ty);
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