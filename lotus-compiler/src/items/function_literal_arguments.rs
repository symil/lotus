use std::slice::from_ref;
use parsable::{DataLocation, parsable};
use crate::program::{ProgramContext, Type, Vasm};
use super::{BlockExpression, Expression, Identifier};

#[parsable]
pub enum FunctionLiteralArguments {
    Single(Identifier),
    #[parsable(brackets="()", separator=",")]
    Multiple(Vec<Identifier>)
}

impl FunctionLiteralArguments {
    pub fn process(&self, type_hint: Option<&Type>, context: &mut ProgramContext) -> &[Identifier] {
        match self {
            FunctionLiteralArguments::Single(name) => from_ref(name),
            FunctionLiteralArguments::Multiple(names) => names.as_slice(),
        }
    }
}