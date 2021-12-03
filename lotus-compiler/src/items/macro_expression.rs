use parsable::{DataLocation, parsable};
use crate::program::{ProgramContext, Vasm, MacroContext, VI, BuiltinType};
use super::{make_string_value_from_literal_unchecked};

#[parsable]
pub struct MacroExpression {
    #[parsable(prefix="#")]
    pub value: MacroExpressionValue
}

#[parsable]
pub enum MacroExpressionValue {
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

impl MacroExpression {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Vasm> {
        let mut m = MacroContext::new(self, context);

        match &self.value {
            MacroExpressionValue::TypeId => m.access_current_type(|type_unwrapped, context| {
                Vasm::new(context.int_type(), vec![], vec![VI::type_id(&type_unwrapped.self_type)])
            }),
            MacroExpressionValue::TypeName => m.access_current_type(|type_unwrapped, context| {
                Vasm::new(context.get_builtin_type(BuiltinType::String, vec![]), vec![], vec![VI::type_name(&type_unwrapped.self_type)])
            }),
            MacroExpressionValue::TypeShortName => m.access_current_type(|type_unwrapped, context| {
                make_string_value_from_literal_unchecked(type_unwrapped.name.as_str(), context)
            }),
            MacroExpressionValue::FieldCount => m.access_current_type(|type_unwrapped, context| {
                Vasm::new(context.int_type(), vec![], vec![VI::int(type_unwrapped.fields.len())])
            }),
            MacroExpressionValue::FieldName => m.access_current_field(|field_info, context| {
                make_string_value_from_literal_unchecked(field_info.name.as_str(), context)
            }),
            MacroExpressionValue::FieldDefaultExpression => m.access_current_field(|field_info, context| {
                field_info.default_value.clone()
            }),
            MacroExpressionValue::VariantCount => m.access_current_type(|type_unwrapped, context| {
                Vasm::new(context.int_type(), vec![], vec![VI::int(type_unwrapped.enum_variants.len())])
            }),
            MacroExpressionValue::VariantName => m.access_current_variant(|variant_info, context| {
                make_string_value_from_literal_unchecked(variant_info.name.as_str(), context)
            }),
            MacroExpressionValue::VariantValue => m.access_current_variant(|variant_info, context| {
                Vasm::new(context.int_type(), vec![], vec![VI::int(variant_info.value)])
            }),
            MacroExpressionValue::AncestorId => m.access_current_ancestor(|ancestor_type, context| {
                Vasm::new(context.int_type(), vec![], vec![VI::type_id(ancestor_type)])
            }),
            MacroExpressionValue::AncestorName => m.access_current_ancestor(|ancestor_type, context| {
                make_string_value_from_literal_unchecked(&ancestor_type.to_string(), context)
            }),
        }
    }
}