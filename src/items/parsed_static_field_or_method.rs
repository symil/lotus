use parsable::parsable;
use crate::{program::{AccessType, FieldKind, ProgramContext, Type, Vasm}, language_server::FieldCompletionOptions};
use super::{ParsedArgumentList, Identifier, ParsedType, process_method_call, process_field_access, ParsedDoubleColonToken};

#[parsable]
pub struct ParsedStaticFieldOrMethod {
    pub ty: ParsedType,
    pub double_colon: ParsedDoubleColonToken,
    pub name: Option<Identifier>,
    pub arguments: Option<ParsedArgumentList>
}

impl ParsedStaticFieldOrMethod {
    pub fn process(&self, type_hint: Option<&Type>, context: &mut ProgramContext) -> Option<Vasm> {
        match self.ty.process(true, type_hint, context) {
            Some(ty) => {
                context.completion_provider.add_static_field_completion(&self.double_colon, &ty, type_hint, Some(&FieldCompletionOptions {
                    show_methods: true,
                    insert_arguments: self.arguments.is_none(),
                    ..Default::default()
                }));

                match &self.name {
                    Some(name) => {
                        context.completion_provider.add_static_field_completion(name, &ty, type_hint, Some(&FieldCompletionOptions {
                            show_methods: true,
                            insert_arguments: self.arguments.is_none(),
                            ..Default::default()
                        }));

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