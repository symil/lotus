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

        if let Some(condition_wasm) = self.condition.process(context) {
            if condition_wasm.ty.is_bool() {
                result = Some(condition_wasm);
            } else {
                context.errors.add(&self.condition, format!("expected `{}`, got `{}`", BuiltinType::Bool.get_name(), &condition_wasm.ty));
            }
        }

        result
    }

    pub fn process_body(&self, context: &mut ProgramContext) -> Option<Vasm> {
        self.statements.process(context)
    }
}