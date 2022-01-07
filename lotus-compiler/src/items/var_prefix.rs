use parsable::{DataLocation, Parsable, parsable};
use crate::{program::{BuiltinType, ProgramContext, Vasm}, wat};

#[parsable]
pub struct VarPrefix {
    pub value: VarPrefixValue
}

#[parsable(impl_display=true)]
#[derive(PartialEq, Clone, Copy)]
pub enum VarPrefixValue {
    // This = "#",
    // Payload = "$",
    System = "@"
}

impl VarPrefix {
    pub fn process(&self, context: &mut ProgramContext) -> Vasm {
        match &self.value {
            VarPrefixValue::System => {
                context.vasm()
                    .set_type(context.get_builtin_type(BuiltinType::System, vec![]))
            },
        }
    }
}