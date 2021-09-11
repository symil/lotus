use std::{collections::HashMap};
use parsable::parsable;
use crate::{generation::{Wat, ToWat, ToWatVec}, program::{IrFragment, ProgramContext, RESULT_VAR_NAME, ScopeKind, Type, VI, Vasm}, wat};
use super::{ActionKeyword, ActionKeywordToken, Expression};

#[parsable]
pub struct Action {
    pub keyword: ActionKeyword,
    pub value: Option<Expression>
}

impl Action {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Vasm> {
        let mut result = None;

        match &self.keyword.token {
            ActionKeywordToken::Return => {
                let function_depth = context.get_scope_depth(ScopeKind::Function).unwrap();
                context.return_found = true;

                let function_blueprint = context.current_function.as_ref().unwrap().borrow();

                match &function_blueprint.return_value {
                    Some(return_value) => match &self.value {
                        Some(expr) => {
                            if let Some(wasm) = expr.process(context) {
                                if return_value.ty.is_assignable_to(&wasm.ty) {
                                    let mut source = vec![wasm];

                                    source.push(Vasm::new(Type::Void, vec![], vec![
                                        VI::set(return_value),
                                        VI::jump(function_depth, None),
                                    ]));

                                    result = Some(IrFragment::merge(Type::Void, source));
                                } else {
                                    if !wasm.ty.is_void() {
                                        context.errors.add(expr, format!("return: expected `{}`, got `{}`", return_value, &wasm.ty));
                                    }
                                }
                            }
                        },
                        None => {
                            context.errors.add(self, format!("return: expected `{}`, got `{}`", return_value::Void));
                        },
                    },
                    None => match &self.value {
                        Some(expr) => {
                            if let Some(wasm) = expr.process(context) {
                                context.errors.add(expr, format!("return: expected `{}`, got `{}`"::Void, &wasm.ty));
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