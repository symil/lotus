use parsable::parsable;

use crate::{generation::Wat, program::{ProgramContext, TypeOld, IrFragment}};

#[parsable(name="boolean")]
pub struct BooleanLiteral {
    #[parsable(regex = r"true|false")]
    pub value: String
}

impl BooleanLiteral {
    pub fn process(&self, context: &mut ProgramContext) -> Option<IrFragment> {
        let i32_value = match self.value.as_str() {
            "true" => 1,
            "false" => 0,
            _ => unreachable!()
        };

        Some(IrFragment::simple(context.bool_type(), Wat::const_i32(i32_value)))
    }
}