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
        let main_type_name = name.process(context)?;
        let main_type = main_type_name.to_main_type();
        let equal = unwrap_item(&self.equal, name, context)?;
        let ty = unwrap_item(&self.ty, equal, context)?;
        let assigned_type = ty.process(true, None, context)?;

        if context.root_tags.disable_main_type_checks {
            context.main_types.set_unchecked(main_type, assigned_type);
        } else if let Err(expected_type) = context.main_types.set(main_type, assigned_type.clone()) {
            context.errors.type_mismatch(ty, &expected_type, &assigned_type);
        }

        Some(())
    }
}