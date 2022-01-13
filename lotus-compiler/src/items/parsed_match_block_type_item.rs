use parsable::parsable;
use crate::program::{ProgramContext, Vasm, IS_METHOD_NAME, EQ_METHOD_NAME};
use super::{ParsedType, Identifier, ParsedDoubleColon};

#[parsable]
pub struct ParsedMatchBlockTypeItem {
    pub ty: ParsedType,
    pub variant: Option<ParsedEnumVariantName>,
}

#[parsable]
pub struct ParsedEnumVariantName {
    pub double_colon: ParsedDoubleColon,
    pub name: Option<Identifier>
}

impl ParsedMatchBlockTypeItem {
    pub fn process(&self, tested_value: Vasm, context: &mut ProgramContext) -> Option<Vasm> {
        let ty = self.ty.process(true, context)?;

        if tested_value.ty.is_object() {
            if !ty.is_object() {
                context.errors.expected_class_type(&self.ty, &ty);
                return None;
            }
        } else if ty != tested_value.ty {
            context.errors.type_mismatch(&self.ty, &tested_value.ty, &ty);
            return None;
        }

        match &self.variant {
            Some(details) => {
                match &details.name {
                    Some(name) => {
                        match ty.get_variant(name.as_str()) {
                            Some(variant_info) => {
                                Some(context.vasm()
                                    .call_static_method(&context.int_type(), EQ_METHOD_NAME, &[], vec![
                                        tested_value,
                                        context.vasm().int(variant_info.value)
                                    ], context)
                                    .set_type(context.bool_type())
                                )
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
                Some(context.vasm()
                    .call_static_method(&ty, IS_METHOD_NAME, &[], vec![tested_value], context)
                    .set_type(context.bool_type())
                )
            },
        }
    }
}