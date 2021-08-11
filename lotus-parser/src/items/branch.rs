use parsable::parsable;
use crate::program::{ProgramContext, Type, Wasm};
use super::{Expression, Statement, StatementList};

#[parsable]
pub struct Branch {
    pub condition: Expression,
    pub statements: StatementList
}

impl Branch {
    pub fn process_condition(&self, context: &mut ProgramContext) -> Option<Wasm> {
        let mut result = None;

        if let Some(wasm) = self.condition.process(context) {
            let mut wat = wasm.wat;

            if wasm.ty.is_boolean() {
                result = Some(Wasm::untyped(wat));
            } else {
                if let Some(convert_wat) = wasm.ty.to_bool() {
                    wat.push(convert_wat);
                    result = Some(Wasm::untyped(wat));
                } else {
                    context.error(&self.condition, format!("branch condition: cannot convert `{}` to `bool`", wasm.ty));
                }
            }
        }

        result
    }

    pub fn process_body(&self, context: &mut ProgramContext) -> Option<Wasm> {
        self.statements.process(context)
    }
}