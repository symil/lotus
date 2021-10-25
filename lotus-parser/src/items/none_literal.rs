use colored::Colorize;
use parsable::parsable;
use crate::{program::{BuiltinType, NONE_FUNC_NAME, ProgramContext, Type, VI, Vasm}, vasm};

#[parsable(name="none")]
pub struct NoneLiteral {
    #[parsable(regex = r"none")]
    pub value: String,
}

impl NoneLiteral {
    pub fn process(&self, type_hint: Option<&Type>, context: &mut ProgramContext) -> Option<Vasm> {
        let mut result = None;

        match type_hint {
            Some(ty) => match ty.get_builtin_type_parameter(BuiltinType::Option) {
                Some(option_type) => {
                    result = Some(Vasm::new(ty.clone(), vec![], vec![VI::call_static_method(option_type, NONE_FUNC_NAME, &[], vasm![], context)]));
                },
                None => {
                    context.errors.add(&self.location, format!("cannot assign `{}` to `{}`", "Option<_>".bold(), ty));
                },
            },
            None => {
                context.errors.add(&self.location, format!("cannot infer `{}` type", "none".bold()));
            },
        }

        result
    }
}