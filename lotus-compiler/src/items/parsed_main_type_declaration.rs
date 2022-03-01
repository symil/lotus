use parsable::{parsable, Parsable};
use crate::program::ProgramContext;
use super::{ParsedHashToken, ParsedMainTypeName, FlexWordItem, ParsedEqualToken, ParsedType, unwrap_item};

#[parsable(cascade = true)]
pub struct ParsedMainTypeDeclaration {
    pub hash: ParsedHashToken,
    pub name: Option<FlexWordItem<ParsedMainTypeName>>,
    pub equal: Option<ParsedEqualToken>,
    pub ty: Option<ParsedType>
}

impl ParsedMainTypeDeclaration {
    pub fn process(&self, context: &mut ProgramContext) -> Option<()> {
        context.completion_provider.add_keyword_completion(&self.hash, ParsedMainTypeName::get_completion_suggestions());

        let name = unwrap_item(&self.name, &self.hash, context)?;
        let assigned_type_name = name.process(context)?;
        let equal = unwrap_item(&self.equal, name, context)?;
        let ty = unwrap_item(&self.ty, equal, context)?;
        let assigned_type = ty.process(true, context)?;

        Some(())
    }
}