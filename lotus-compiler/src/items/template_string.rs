use parsable::parsable;
use crate::program::{ProgramContext, Vasm};

use super::TemplateStringFragment;

#[parsable]
pub struct TemplateString {
    #[parsable(brackets="``")]
    pub fragments: Vec<TemplateStringFragment>
}

impl TemplateString {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Vasm> {
        todo!()
    }
}