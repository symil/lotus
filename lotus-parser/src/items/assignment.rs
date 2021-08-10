use std::collections::HashMap;

use parsable::parsable;
use crate::{generation::Wat, program::{AccessType, ProgramContext, Wasm}};
use super::{AssignmentOperator, Expression, VarPath};

#[parsable]
pub struct Assignment {
    pub lvalue: VarPath,
    pub rvalue: Option<(AssignmentOperator, Expression)>
}

impl Assignment {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Wasm> {
        let mut result = None;

        if let Some((equal_token, rvalue)) = &self.rvalue {
            let left_wasm_opt = self.lvalue.process(AccessType::Set(&equal_token), context);
            let right_wasm_opt = rvalue.process(context);

            if let Some(left_wasm) = left_wasm_opt {
                if let Some(right_wasm) = right_wasm_opt {
                    if left_wasm.ty.is_assignable(&right_wasm.ty, context, &mut HashMap::new()) {
                        let mut wat = vec![];

                        wat.extend(right_wasm.wat);
                        wat.extend(left_wasm.wat);
                        wat.push(Wat::inst("drop"));

                        result = Some(Wasm::untyped(wat));
                    } else {
                        context.error(rvalue, format!("assignment: right-hand side type `{}` does not match left-hand side type `{}`", &right_wasm.ty, &left_wasm.ty));
                    }
                }
            }
        } else {
            if let Some(wasm) = self.lvalue.process(AccessType::Get, context) {
                let mut wat = vec![];

                wat.extend(wasm.wat);

                if !wasm.ty.is_void() {
                    wat.push(Wat::inst("drop"));
                }

                result = Some(Wasm::untyped(wat));
            }
        }

        result
    }
}