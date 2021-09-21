use std::{collections::HashMap};
use parsable::parsable;
use crate::{program::{Vasm, ProgramContext, RESULT_VAR_NAME, ScopeKind, Type, VI}, vasm, wat};
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

                let function_wrapped = context.get_current_function().unwrap();
                let function_unwrapped = function_wrapped.borrow();

                match &function_unwrapped.return_value {
                    Some(return_value) => match &self.value {
                        Some(expr) => {
                            if let Some(vasm) = expr.process(context) {
                                if return_value.ty.is_assignable_to(&vasm.ty) {
                                    result = Some(vasm![
                                        VI::set(return_value, vasm),
                                        VI::jump(function_depth)
                                    ]);
                                } else {
                                    if !vasm.ty.is_undefined() {
                                        context.errors.add(expr, format!("expected `{}`, got `{}`", &return_value.ty, &vasm.ty));
                                    }
                                }
                            }
                        },
                        None => {
                            context.errors.add(self, format!("expected `{}`, got `{}`", &return_value.ty, Type::Void));
                        },
                    },
                    None => match &self.value {
                        Some(expr) => {
                            if let Some(vasm) = expr.process(context) {
                                context.errors.add(expr, format!("expected `{}`, got `{}`", Type::Void, &vasm.ty));
                            }
                        },
                        None => {
                            result = Some(vasm![VI::jump(function_depth)]);
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
                                ActionKeywordToken::Break => Some(vasm![VI::jump(depth + 1)]),
                                ActionKeywordToken::Continue => Some(vasm![VI::jump(depth)]),
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