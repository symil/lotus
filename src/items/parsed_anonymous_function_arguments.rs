use std::slice::from_ref;
use parsable::{ItemLocation, parsable};
use crate::program::{ProgramContext, Type, Vasm, VariableInfo};
use super::{ParsedBlockExpression, ParsedExpression, Identifier, ParsedType};

#[parsable]
pub enum ParsedAnonymousFunctionArguments {
    Single(Identifier),
    #[parsable(brackets="()", separator=",")]
    Multiple(Vec<ParsedAnonymousFunctionArgument>)
}

#[parsable]
pub struct ParsedAnonymousFunctionArgument {
    pub name: Identifier,
    #[parsable(prefix=":")]
    pub ty: Option<ParsedType>
}

impl ParsedAnonymousFunctionArguments {
    pub fn process(&self, type_hint: Option<&Type>, context: &mut ProgramContext) -> Vec<(&Identifier, Option<Type>)> {
        let mut result = vec![];

        match self {
            ParsedAnonymousFunctionArguments::Single(name) => {
                result.push((name, None));
            },
            ParsedAnonymousFunctionArguments::Multiple(arguments) => {
                for arg in arguments {
                    let name = &arg.name;
                    let ty = arg.ty.as_ref().and_then(|ty| ty.process(true, None, context));

                    result.push((name, ty));
                }
            },
        };

        result
    }
}