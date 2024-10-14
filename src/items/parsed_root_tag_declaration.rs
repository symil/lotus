use parsable::{parsable, Parsable};
use crate::program::ProgramContext;
use super::{ParsedAtToken, ParsedRootTagName, FlexWordItem, unwrap_item};

#[parsable]
pub struct ParsedRootTagDeclaration {
    pub at: ParsedAtToken,
    pub tag_name: Option<FlexWordItem<ParsedRootTagName>>,
}

impl ParsedRootTagDeclaration {
    pub fn process(&self, context: &mut ProgramContext) -> Option<()> {
        context.completion_provider.add_keyword_completion(&self.at, ParsedRootTagName::get_completion_suggestions());

        let tag_name = unwrap_item(&self.tag_name, self, context)?.process(context)?;

        match tag_name {
            ParsedRootTagName::DisableCheckMainType => context.root_tags.check_main_types = true,
            ParsedRootTagName::EnableCheckFieldAccess => context.root_tags.check_field_access = true,
        }

        Some(())
    }
}