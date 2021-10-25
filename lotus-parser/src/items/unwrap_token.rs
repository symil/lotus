use parsable::parsable;
use crate::program::{BuiltinInterface, ProgramContext, Type, Vasm};

#[parsable]
pub struct UnwrapToken {
    #[parsable(regex = r"\?")]
    pub value: String
}

impl UnwrapToken {
    pub fn process(&self, parent_type: &Type, context: &mut ProgramContext) -> Option<Vasm> {
        parent_type.call_builtin_interface_no_arg(self, BuiltinInterface::Unwrap, context)
    }
}