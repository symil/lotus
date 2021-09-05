use std::{collections::HashMap};
use parsable::parsable;
use crate::{generation::{Wat, ToWat, ToWatVec}, program::{ProgramContext, RESULT_VAR_NAME, ScopeKind, TypeOld, Wasm}, wat};
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
                                if return_type.is_assignable_to(&wasm.ty, context, &mut HashMap::new()) {
                                    let mut source = vec![wasm];

                                    source.push(Wasm::new(TypeOld::Void, vec![
                                        Wat::set_local_from_stack(RESULT_VAR_NAME),
                                        Wat::new("br", function_depth)
                                    ], vec![]));

                                    result = Some(Wasm::merge(TypeOld::Void, source));
                                } else {
                                    if !wasm.ty.is_void() {
                                        context.errors.add(expr, format!("return: expected `{}`, got `{}`", return_type, &wasm.ty));
                                    }
                                }
                            }
                        },
                        None => {
                            context.errors.add(self, format!("return: expected `{}`, got `{}`", return_type, TypeOld::Void));
                        },
                    },
                    None => match &self.value {
                        Some(expr) => {
                            if let Some(wasm) = expr.process(context) {
                                context.errors.add(expr, format!("return: expected `{}`, got `{}`", TypeOld::Void, &wasm.ty));
                            }
                        },
                        None => {
                            result = Some(Wasm::new(TypeOld::Void, Wat::new("br", function_depth), vec![]));
                        },
                    },
                }
            },
            ActionKeywordToken::Break | ActionKeywordToken::Continue => {
                if let Some(value) = &self.value {
                    value.process(context);
                    context.errors.add(value, format!("keyword `{}` must not be followed by an expression", &self.keyword.token));
                } else {
                    match context.get_scope_depth(ScopeKind::Loop) {
                        Some(depth) => {
                            result = match &self.keyword.token {
                                ActionKeywordToken::Break => Some(Wasm::new(TypeOld::Void, wat!["br", depth + 1], vec![])),
                                ActionKeywordToken::Continue => Some(Wasm::new(TypeOld::Void, wat!["br", depth], vec![])),
                                _ => unreachable!()
                            }
                        },
                        None => {
                            context.errors.add(&self.keyword, format!("keyword `{}` can only be used from inside a loop", &self.keyword.token));
                        }
                    }
                }
            },
        }

        result
    }
}