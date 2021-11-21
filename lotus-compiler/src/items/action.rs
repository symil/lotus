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
                let function_wrapped = context.get_current_function().unwrap();
                let function_unwrapped = function_wrapped.borrow();
                let return_type = function_unwrapped.return_type.clone();

                match return_type.is_void() {
                    false => match &self.value {
                        Some(expr) => {
                            let type_hint = Some(&return_type);

                            if let Some(vasm) = expr.process(type_hint, context) {
                                if vasm.ty.is_assignable_to(&return_type) {
                                    result = Some(vasm![
                                        VI::return_value(vasm)
                                    ]);
                                } else {
                                    if !vasm.ty.is_undefined() {
                                        context.errors.add(expr, format!("expected `{}`, got `{}`", &return_type, &vasm.ty));
                                    }
                                }
                            }
                        },
                        None => {
                            context.errors.add(self, format!("expected `{}`, got `{}`", &return_type, context.void_type()));
                        },
                    },
                    true => match &self.value {
                        Some(expr) => {
                            if let Some(vasm) = expr.process(None, context) {
                                context.errors.add(expr, format!("expected `{}`, got `{}`", context.void_type(), &vasm.ty));
                            }
                        },
                        None => {
                            result = Some(vasm![VI::return_value(vasm![])]);
                        },
                    },
                }
            },
            ActionKeywordToken::Break | ActionKeywordToken::Continue => {
                if let Some(value) = &self.value {
                    value.process(None, context);
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