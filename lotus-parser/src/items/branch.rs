use parsable::parsable;
use crate::program::{ProgramContext, Type, Wasm};
use super::{Expression, Statement, StatementList};

#[parsable]
pub struct Branch {
    #[parsable(set_marker="no-object")]
    pub condition: Expression,
    pub statements: StatementList
}

impl Branch {
    pub fn process_condition(&self, context: &mut ProgramContext) -> Option<Wasm> {
        let mut result = None;

        if let Some(wasm) = self.condition.process(context) {
            let mut source = vec![];

            if wasm.ty.is_boolean() {
                source.push(wasm);
                result = Some(Wasm::merge(Type::Boolean, source));
            } else {
                if let Some(convert_wasm) = wasm.ty.to_bool() {
                    source.push(wasm);
                    source.push(convert_wasm);
                    result = Some(Wasm::merge(Type::Boolean, source));
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