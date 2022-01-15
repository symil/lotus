use parsable::{ItemLocation, Parsable, parsable};
use crate::{program::{BuiltinType, ProgramContext, Vasm}, wat};

#[parsable]
pub struct ParsedVarPrefix {
    pub token: ParsedVarPrefixToken
}

#[parsable(impl_display=true)]
#[derive(PartialEq, Clone, Copy)]
pub enum ParsedVarPrefixToken {
    // This = "#",
    // Payload = "$",
    System = "@"
}

impl ParsedVarPrefix {
    pub fn process(&self, context: &mut ProgramContext) -> Vasm {
        match &self.token {
            ParsedVarPrefixToken::System => {
                context.vasm()
                    .set_type(context.get_builtin_type(BuiltinType::System, vec![]))
            },
        }
    }
}