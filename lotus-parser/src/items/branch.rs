use parsable::parsable;
use crate::program::{BuiltinInterface, ProgramContext, TypeOld, IrFragment};
use super::{Expression, Statement, StatementList};

#[parsable]
pub struct Branch {
    #[parsable(set_marker="no-object")]
    pub condition: Expression,
    pub statements: StatementList
}

impl Branch {
    pub fn process_condition(&self, context: &mut ProgramContext) -> Option<IrFragment> {
        let mut result = None;

        if let Some(condition_wasm) = self.condition.process(context) {
            if let Some(to_bool_wasm) = context.call_builtin_interface_no_arg(&self.condition, BuiltinInterface::ToBool, &condition_wasm.ty) {
                result = Some(IrFragment::merge(context.bool_type(), vec![condition_wasm, to_bool_wasm]));
            }
        }

        result
    }

    pub fn process_body(&self, context: &mut ProgramContext) -> Option<IrFragment> {
        self.statements.process(context)
    }
}