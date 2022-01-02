use std::ptr::NonNull;
use parsable::{DataLocation, Parsable, parsable};
use crate::{program::{BuiltinType, ProgramContext, Vasm}, wat};

#[parsable]
pub struct VarPrefixWrapper {
    pub value: VarPrefix
}

#[parsable(impl_display=true)]
#[derive(PartialEq, Clone, Copy)]
pub enum VarPrefix {
    // This = "#",
    // Payload = "$",
    System = "@"
}

impl VarPrefixWrapper {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Vasm> {
        let mut result = None;

        match &self.value {
            VarPrefix::System => {
                result = Some(context.vasm()
                    .set_type(context.get_builtin_type(BuiltinType::System, vec![]))
                )
            },
        };

        result
    }
}