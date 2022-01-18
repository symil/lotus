use parsable::{parsable, Parsable};
use crate::program::{ProgramContext, Type, EXTENDS_KEYWORD};
use super::{Identifier, ParsedType, unwrap_item};

#[parsable]
pub struct ParsedTypeExtend {
    pub extends: Identifier,
    pub ty: Option<ParsedType>
}

impl ParsedTypeExtend {
    pub fn collect_instancied_type_names(&self, list: &mut Vec<String>, context: &mut ProgramContext) {
        if let Some(ty) = &self.ty {
            ty.collect_instancied_type_names(list, context);
        }
    }

    pub fn process(&self, context: &mut ProgramContext) -> Option<Type> {
        context.completion_provider.add_keyword_completion(&self.extends, &[EXTENDS_KEYWORD]);
        
        if self.extends.as_str() != EXTENDS_KEYWORD {
            context.errors.expected_keyword(self, EXTENDS_KEYWORD);
            return None;
        }

        let ty = unwrap_item(&self.ty, self, context)?;

        ty.process(false, context)
    }
}