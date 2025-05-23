use colored::Colorize;
use parsable::parsable;
use crate::{program::{BuiltinType, NONE_METHOD_NAME, ProgramContext, Type, Vasm}};

#[parsable(name="none")]
pub struct ParsedNoneLiteral {
    #[parsable(regex = r"none\b")]
    pub token: String,
}

impl ParsedNoneLiteral {
    pub fn process(&self, type_hint: Option<&Type>, context: &mut ProgramContext) -> Option<Vasm> {
        let mut result = None;

        match type_hint {
            Some(ty) => {
                result = Some(context.vasm()
                    .call_static_method(ty, NONE_METHOD_NAME, &[], vec![], context)
                    .set_type(ty)
                );
            },
            None => {
                context.errors.generic(&self.location, format!("cannot infer `{}` type", "none".bold()));
            },
        }

        result
    }
}