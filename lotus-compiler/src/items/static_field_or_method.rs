use parsable::parsable;
use crate::{program::{AccessType, FieldKind, ProgramContext, Type, Vasm}};
use super::{ArgumentList, Identifier, ParsedType, process_method_call, process_field_access};

#[parsable]
pub struct StaticFieldOrMethod {
    pub ty: ParsedType,
    pub double_colon: DoubleColonToken,
    pub name: Option<Identifier>,
    pub arguments: Option<ArgumentList>
}

#[parsable]
pub struct DoubleColonToken {
    #[parsable(value="::")]
    pub double_colon: String
}

impl StaticFieldOrMethod {
    pub fn process(&self, type_hint: Option<&Type>, context: &mut ProgramContext) -> Option<Vasm> {
        match self.ty.process(true, context) {
            Some(ty) => {
                context.add_static_field_completion_area(&self.double_colon, &ty, self.arguments.is_none());

                match &self.name {
                    Some(name) => {
                        context.add_static_field_completion_area(name, &ty, self.arguments.is_none());

                        match &self.arguments {
                            Some(args) => process_method_call(&ty, FieldKind::Static, name, &[], args, type_hint, AccessType::Get, context),
                            None => process_field_access(&ty, FieldKind::Static, name, AccessType::Get, context),
                        }
                    },
                    None => {
                        context.errors.expected_identifier(&self.double_colon);
                        None
                    },
                }
            },
            None => None,
        }
    }
}