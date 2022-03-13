use parsable::{parsable, Parsable};
use crate::program::{ProgramContext, Type, EXTENDS_KEYWORD};
use super::{Identifier, ParsedType, unwrap_item, FlexKeyword};

#[parsable]
pub struct ParsedTypeExtend {
    pub extends: FlexKeyword<EXTENDS_KEYWORD>,
    pub ty: Option<ParsedType>
}

impl ParsedTypeExtend {
    pub fn collect_instancied_type_names(&self, list: &mut Vec<String>, context: &mut ProgramContext) {
        if let Some(ty) = &self.ty {
            ty.collect_instancied_type_names(list, context);
        }
    }

    pub fn process(&self, context: &mut ProgramContext) -> Option<Type> {
        let keyword = self.extends.process(context)?;
        let ty = unwrap_item(&self.ty, self, context)?;

        ty.process(false, None, context)
    }
}