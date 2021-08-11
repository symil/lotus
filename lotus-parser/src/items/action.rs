use std::collections::HashMap;

use parsable::parsable;
use crate::{generation::{RESULT_VAR_NAME, Wat}, program::{ProgramContext, Type, Wasm}};
use super::{ActionKeyword, Expression};

#[parsable]
pub struct Action {
    pub keyword: ActionKeyword,
    pub value: Option<Expression>
}

impl Action {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Wasm> {
        let mut result = None;

        match &self.keyword {
            ActionKeyword::Return => {
                match context.function_return_type.clone() {
                    Some(return_type) => match &self.value {
                        Some(expr) => {
                            if let Some(wasm) = expr.process(context) {
                                if return_type.is_assignable(&wasm.ty, context, &mut HashMap::new()) {
                                    let mut wat = vec![];

                                    wat.extend(wasm.wat);
                                    wat.push(Wat::set_local_from_stack(RESULT_VAR_NAME));
                                    wat.push(Wat::new("br", context.function_depth));
                                    
                                    context.return_found = true;

                                    result = Some(Wasm::untyped(wat));
                                } else {
                                    context.error(expr, format!("return: expected `{}`, got `{}`", return_type, &wasm.ty));
                                }
                            }
                        },
                        None => {
                            context.error(self, format!("return: expected `{}`, got `{}`", return_type, Type::Void));
                        },
                    },
                    None => match &self.value {
                        Some(expr) => {
                            if let Some(wasm) = expr.process(context) {
                                context.error(expr, format!("return: expected `{}`, got `{}`", Type::Void, &wasm.ty));
                            }
                        },
                        None => {
                            result = Some(Wasm::untyped(Wat::new("br", context.function_depth)));
                        },
                    },
                }
            },
        }

        result
    }
}