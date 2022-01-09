use parsable::{DataLocation, parsable};
use crate::program::{ProgramContext, Vasm, Type, IS_METHOD_NAME, VariableInfo};
use super::{ParsedType, Identifier, ParsedVarDeclarationNames};

#[parsable]
pub struct ParsedAsOperation {
    pub keyword: ParsedAsKeyword,
    pub ty: Option<ParsedType>,
}

#[parsable]
pub struct ParsedAsKeyword {
    #[parsable(value="as")]
    pub token: String
}

impl ParsedAsOperation {
    pub fn process(&self, source_type: &Type, context: &mut ProgramContext) -> Option<Vasm> {
        let target_type = match &self.ty {
            Some(parsed_type) => match parsed_type.process(true, context) {
                Some(ty) => ty,
                None => {
                    return None;
                },
            },
            None => {
                context.errors.expected_identifier(&self.keyword);
                return None;
            },
        };

        Some(context.vasm()
            .set_type(target_type)
        )
    }
}