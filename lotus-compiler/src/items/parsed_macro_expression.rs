use parsable::{ItemLocation, parsable};
use crate::program::{ProgramContext, Vasm, MacroContext, BuiltinType};
use super::{make_string_value_from_literal_unchecked};

#[parsable]
pub struct ParsedMacroExpression {
    #[parsable(prefix="#")]
    pub token: MacroExpressionToken
}

#[parsable]
pub enum MacroExpressionToken {
    Line = "LINE",
    TypeId = "TYPE_ID",
    TypeName = "TYPE_NAME",
    TypeShortName = "TYPE_SHORT_NAME",
    FieldCount = "FIELD_COUNT",
    FieldName = "FIELD_NAME",
    FieldDefaultExpression = "FIELD_DEFAULT_EXPRESSION",
    VariantCount = "VARIANT_COUNT",
    VariantName = "VARIANT_NAME",
    VariantValue = "VARIANT_VALUE",
    AncestorId = "ANCESTOR_ID",
    AncestorName = "ANCESTOR_NAME"
}

impl ParsedMacroExpression {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Vasm> {
        let mut m = MacroContext::new(self, context);

        match &self.token {
            MacroExpressionToken::Line => {
                Some(context.vasm()
                    .int(self.location.get_start_line_col().0)
                    .set_type(context.int_type())
                )
            }
            MacroExpressionToken::TypeId => m.access_current_type(|type_unwrapped, context| {
                context.vasm()
                    .type_id(&type_unwrapped.self_type)
                    .set_type(context.int_type())
            }, context),
            MacroExpressionToken::TypeName => m.access_current_type(|type_unwrapped, context| {
                context.vasm()
                    .type_name(&type_unwrapped.self_type)
                    .set_type(context.get_builtin_type(BuiltinType::String, vec![]))
            }, context),
            MacroExpressionToken::TypeShortName => m.access_current_type(|type_unwrapped, context| {
                make_string_value_from_literal_unchecked(type_unwrapped.name.as_str(), context)
            }, context),
            MacroExpressionToken::FieldCount => m.access_current_type(|type_unwrapped, context| {
                context.vasm()
                    .int(type_unwrapped.fields.len())
                    .set_type(context.int_type())
            }, context),
            MacroExpressionToken::FieldName => m.access_current_field(|field_info, context| {
                make_string_value_from_literal_unchecked(field_info.name.as_str(), context)
            }, context),
            MacroExpressionToken::FieldDefaultExpression => m.access_current_field(|field_info, context| {
                field_info.default_value.clone()
            }, context),
            MacroExpressionToken::VariantCount => m.access_current_type(|type_unwrapped, context| {
                context.vasm()
                    .int(type_unwrapped.enum_variants.len())
                    .set_type(context.int_type())
            }, context),
            MacroExpressionToken::VariantName => m.access_current_variant(|variant_info, context| {
                make_string_value_from_literal_unchecked(variant_info.name.as_str(), context)
            }, context),
            MacroExpressionToken::VariantValue => m.access_current_variant(|variant_info, context| {
                context.vasm()
                    .int(variant_info.value)
                    .set_type(context.int_type())
            }, context),
            MacroExpressionToken::AncestorId => m.access_current_ancestor(|ancestor_type, context| {
                context.vasm()
                    .type_id(ancestor_type)
                    .set_type(context.int_type())
            }, context),
            MacroExpressionToken::AncestorName => m.access_current_ancestor(|ancestor_type, context| {
                make_string_value_from_literal_unchecked(&ancestor_type.to_string(), context)
            }, context),
        }
    }
}