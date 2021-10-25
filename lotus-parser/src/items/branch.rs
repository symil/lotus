use parsable::parsable;
use crate::program::{BuiltinInterface, BuiltinType, ProgramContext, Vasm};
use super::{Expression, Statement, StatementList};

#[parsable]
pub struct Branch {
    #[parsable(set_marker="no-object")]
    pub condition: Expression,
    pub statements: StatementList
}

impl Branch {
    pub fn process_condition(&self, context: &mut ProgramContext) -> Option<Vasm> {
        let mut result = None;

        if let Some(condition_wasm) = self.condition.process(None, context) {
            if let Some(convert_vasm) = condition_wasm.ty.call_builtin_interface_no_arg(self, BuiltinInterface::ToBool, context) {
                result = Some(Vasm::merge(vec![condition_wasm, convert_vasm]));
            }
        }

        result
    }

    pub fn process_body(&self, context: &mut ProgramContext) -> Option<Vasm> {
        self.statements.process(context)
    }
}