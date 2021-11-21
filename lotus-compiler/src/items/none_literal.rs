use colored::Colorize;
use parsable::parsable;
use crate::{program::{BuiltinType, NONE_METHOD_NAME, ProgramContext, Type, VI, Vasm}, vasm};

#[parsable(name="none")]
pub struct NoneLiteral {
    #[parsable(regex = r"none\b")]
    pub value: String,
}

impl NoneLiteral {
    pub fn process(&self, type_hint: Option<&Type>, context: &mut ProgramContext) -> Option<Vasm> {
        let mut result = None;

        match type_hint {
            Some(ty) => {
                result = Some(Vasm::new(ty.clone(), vec![], vec![VI::call_static_method(ty, NONE_METHOD_NAME, &[], vasm![], context)]));
            },
            None => {
                dbg!(type_hint);
                context.errors.add(&self.location, format!("cannot infer `{}` type", "none".bold()));
            },
        }

        result
    }
}