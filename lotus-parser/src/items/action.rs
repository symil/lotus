use std::{collections::HashMap};
use parsable::parsable;
use crate::{generation::{Wat, ToWat, ToWatVec}, program::{Vasm, ProgramContext, RESULT_VAR_NAME, ScopeKind, Type, VI}, wat};
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
                            if let Some(vasm) = expr.process(context) {
                                if return_value.ty.is_assignable_to(&vasm.ty) {
                                    let mut source = vec![vasm];

                                    source.push(Vasm::new(Type::Void, vec![], vec![
                                        VI::set(return_value),
                                        VI::jump(function_depth, None),
                                    ]));

                                    result = Some(Vasm::merge(source));
                                } else {
                                    if !vasm.ty.is_void() {
                                        context.errors.add(expr, format!("return: expected `{}`, got `{}`", &return_value.ty, &vasm.ty));
                                    }
                                }
                            }
                        },
                        None => {
                            context.errors.add(self, format!("return: expected `{}`, got `{}`", &return_value.ty, Type::Void));
                        },
                    },
                    None => match &self.value {
                        Some(expr) => {
                            if let Some(vasm) = expr.process(context) {
                                context.errors.add(expr, format!("return: expected `{}`, got `{}`", Type::Void, &vasm.ty));
                            }
                        },
                        None => {
                            result = Some(Vasm::new(Type::Void, vec![], vec![VI::jump(function_depth, None)]));
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
                                ActionKeywordToken::Break => Some(Vasm::new(Type::Void, vec![], vec![VI::jump(depth + 1, None)])),
                                ActionKeywordToken::Continue => Some(Vasm::new(Type::Void, vec![], vec![VI::jump(depth, None)])),
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