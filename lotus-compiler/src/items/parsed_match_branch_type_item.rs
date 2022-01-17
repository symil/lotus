use parsable::parsable;
use crate::program::{ProgramContext, Vasm, IS_METHOD_NAME, EQ_METHOD_NAME, Type};
use super::{ParsedType, Identifier, ParsedDoubleColonToken};

#[parsable]
pub struct ParsedMatchBranchTypeItem {
    pub ty: ParsedType,
    pub variant: Option<ParsedEnumVariantName>,
}

#[parsable]
pub struct ParsedEnumVariantName {
    pub double_colon: ParsedDoubleColonToken,
    pub name: Option<Identifier>
}

impl ParsedMatchBranchTypeItem {
    pub fn process(&self, tested_value: Vasm, context: &mut ProgramContext) -> Option<(Type, Vasm)> {
        let ty = self.ty.process(true, context)?;

        if tested_value.ty.is_object() {
            if !ty.is_object() {
                context.errors.expected_class_type(&self.ty, &ty);
                return None;
            }

            if let Some(variant) = &self.variant {
                context.errors.generic(variant, format!("unexpected enum variant"));
                return None;
            }

        } else {
            if ty != tested_value.ty {
                context.errors.type_mismatch(&self.ty, &tested_value.ty, &ty);
                return None;
            }

            if self.variant.is_none() {
                context.errors.expected_token(self, "::");
                return None;
            }
        }

        match &self.variant {
            Some(details) => {
                context.completion_provider.add_enum_variant_completion(&details.double_colon, &ty);

                match &details.name {
                    Some(name) => {
                        context.completion_provider.add_enum_variant_completion(name, &ty);

                        match ty.get_variant(name.as_str()) {
                            Some(variant_info) => {
                                Some((
                                    ty.clone(),
                                    context.vasm()
                                        .append(tested_value)
                                        .call_regular_method(&context.int_type(), EQ_METHOD_NAME, &[], vec![
                                            context.vasm().int(variant_info.value)
                                        ], context)
                                        .set_type(context.bool_type())
                                ))
                            },
                            None => {
                                context.errors.generic(name, format!("type `{}` has no enum variant `{}`", &ty, name.as_str()));
                                None
                            },
                        }
                    },
                    None => {
                        context.errors.expected_identifier(details);
                        None
                    },
                }
            },
            None => {
                Some((
                    ty.clone(),
                    context.vasm()
                        .call_static_method(&ty, IS_METHOD_NAME, &[], vec![tested_value], context)
                        .set_type(context.bool_type())
                ))
            },
        }
    }
}