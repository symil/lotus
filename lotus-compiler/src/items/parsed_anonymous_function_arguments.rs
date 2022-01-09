use std::slice::from_ref;
use parsable::{DataLocation, parsable};
use crate::program::{ProgramContext, Type, Vasm};
use super::{ParsedBlockExpression, ParsedExpression, Identifier};

#[parsable]
pub enum ParsedAnonymousFunctionArguments {
    Single(Identifier),
    #[parsable(brackets="()", separator=",")]
    Multiple(Vec<Identifier>)
}

impl ParsedAnonymousFunctionArguments {
    pub fn process(&self, type_hint: Option<&Type>, context: &mut ProgramContext) -> &[Identifier] {
        match self {
            ParsedAnonymousFunctionArguments::Single(name) => from_ref(name),
            ParsedAnonymousFunctionArguments::Multiple(names) => names.as_slice(),
        }
    }
}