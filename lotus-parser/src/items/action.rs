use std::{collections::HashMap};
use parsable::parsable;
use crate::{generation::{RESULT_VAR_NAME, Wat, ToWat, ToWatVec}, program::{ProgramContext, ScopeKind, Type, Wasm}, wat};
use super::{ActionKeyword, ActionKeywordToken, Expression};

#[parsable]
pub struct Action {
    pub keyword: ActionKeyword,
    pub value: Option<Expression>
}

impl Action {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Wasm> {
        let mut result = None;

        match &self.keyword.token {
            ActionKeywordToken::Return => {
                let function_depth = context.get_scope_depth(ScopeKind::Function).unwrap();
                context.return_found = true;

                match context.function_return_type.clone() {
                    Some(return_type) => match &self.value {
                        Some(expr) => {
                            if let Some(wasm) = expr.process(context) {
                                if return_type.is_assignable(&wasm.ty, context, &mut HashMap::new()) {
                                    let mut wat = vec![];

                                    wat.extend(wasm.wat);
                                    wat.push(Wat::set_local_from_stack(RESULT_VAR_NAME));
                                    wat.push(Wat::new("br", function_depth));

                                    result = Some(Wasm::new(Type::Void, wat, vec![]));
                                } else {
                                    if !wasm.ty.is_void() {
                                        context.error(expr, format!("return: expected `{}`, got `{}`", return_type, &wasm.ty));
                                    }
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
                            result = Some(Wasm::new(Type::Void, Wat::new("br", function_depth), vec![]));
                        },
                    },
                }
            },
            ActionKeywordToken::Break | ActionKeywordToken::Continue => {
                if let Some(value) = &self.value {
                    value.process(context);
                    context.error(value, format!("keyword `{}` must not be followed by an expression", &self.keyword.token));
                } else {
                    match context.get_scope_depth(ScopeKind::Loop) {
                        Some(depth) => {
                            result = match &self.keyword.token {
                                ActionKeywordToken::Break => Some(Wasm::new(Type::Void, wat!["br", depth + 1], vec![])),
                                ActionKeywordToken::Continue => Some(Wasm::new(Type::Void, wat!["br", depth], vec![])),
                                _ => unreachable!()
                            }
                        },
                        None => {
                            context.error(&self.keyword, format!("keyword `{}` can only be used from inside a loop", &self.keyword.token));
                        }
                    }
                }
            },
        }

        result
    }
}