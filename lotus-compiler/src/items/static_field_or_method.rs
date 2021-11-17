use parsable::parsable;
use crate::{program::{AccessType, FieldKind, ProgramContext, Type, Vasm}};
use super::{ArgumentList, Identifier, ParsedType, ParsedTypeWrapper, process_method_call, process_field_access};

#[parsable]
pub struct StaticFieldOrMethod {
    pub ty: ParsedTypeWrapper,
    #[parsable(prefix="::")]
    pub name: Identifier,
    pub args: Option<ArgumentList>
}

impl StaticFieldOrMethod {
    pub fn process(&self, type_hint: Option<&Type>, context: &mut ProgramContext) -> Option<Vasm> {
        match self.ty.process(true, context) {
            Some(ty) => match &self.args {
                Some(args) => process_method_call(&ty, FieldKind::Static, &self.name, &[], args, type_hint, AccessType::Get, context),
                None => process_field_access(&ty, FieldKind::Static, &self.name, AccessType::Get, context),
            },
            None => None,
        }
    }
}