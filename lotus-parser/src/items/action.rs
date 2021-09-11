use std::{collections::HashMap};
use parsable::parsable;
use crate::{generation::{Wat, ToWat, ToWatVec}, program::{ProgramContext, RESULT_VAR_NAME, ScopeKind, Type, TypeOld, IrFragment}, wat};
use super::{ActionKeyword, ActionKeywordToken, Expression};

#[parsable]
pub struct Action {
    pub keyword: ActionKeyword,
    pub value: Option<Expression>
}

impl Action {
    pub fn process(&self, context: &mut ProgramContext) -> Option<IrFragment> {
        let mut result = None;

        match &self.keyword.token {
            ActionKeywordToken::Return => {
                let function_depth = context.get_scope_depth(ScopeKind::Function).unwrap();
                context.return_found = true;

                let function_blueprint = context.current_function.as_ref().unwrap().borrow();

                match &function_blueprint.return_type {
                    Some(return_type) => match &self.value {
                        Some(expr) => {
                            if let Some(wasm) = expr.process(context) {
                                if return_type.is_assignable_to(&wasm.ty) {
                                    let mut source = vec![wasm];

                                    source.push(IrFragment::new(Type::Void, vec![
                                        Wat::set_local_from_stack(RESULT_VAR_NAME),
                                        Wat::new("br", function_depth)
                                    ], vec![]));

                                    result = Some(IrFragment::merge(Type::Void, source));
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
                            result = Some(IrFragment::new(Type::Void, Wat::new("br", function_depth), vec![]));
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
                                ActionKeywordToken::Break => Some(IrFragment::new(Type::Void, wat!["br", depth + 1], vec![])),
                                ActionKeywordToken::Continue => Some(IrFragment::new(Type::Void, wat!["br", depth], vec![])),
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