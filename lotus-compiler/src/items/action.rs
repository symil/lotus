use std::{collections::HashMap};
use parsable::parsable;
use crate::{program::{CompilationError, ProgramContext, ScopeKind, Type, VI, Vasm}, vasm, wat};
use super::{ActionKeywordWrapper, ActionKeyword, Expression};

#[parsable]
pub struct Action {
    pub keyword: ActionKeywordWrapper,
    pub value: Option<Expression>
}

impl Action {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Vasm> {
        match &self.keyword.value {
            ActionKeyword::Return => {
                let function_wrapped = context.get_current_function().unwrap();
                let return_type = function_wrapped.borrow().signature.return_type.clone();

                match return_type.is_void() {
                    true => {
                        match &self.value {
                            Some(expr) => {
                                match expr.process(None, context) {
                                    Some(vasm) => match vasm.ty.is_void() {
                                        true => Some(vasm),
                                        false => context.errors.add_and_none(CompilationError::type_mismatch(expr, &context.void_type(), &vasm.ty)),
                                    },
                                    None => None,
                                }
                            },
                            None => {
                                Some(vasm![VI::return_value(vasm![])])
                            },
                        }
                    },
                    false => {
                        match &self.value {
                            Some(expr) => match expr.process(Some(&return_type), context) {
                                Some(vasm) => match vasm.ty.is_assignable_to(&return_type) {
                                    true => Some(vasm![ VI::return_value(vasm) ]),
                                    false => context.errors.add_and_none(CompilationError::type_mismatch(expr, &return_type, &vasm.ty)),
                                },
                                None => None,
                            },
                            None => context.errors.add_and_none(CompilationError::type_mismatch(self, &return_type, &context.void_type()))
                        }
                    },
                }
            },
            ActionKeyword::Check => {
                match &self.value {
                    Some(value) => todo!(),
                    None => todo!(),
                }
            },
            ActionKeyword::Break | ActionKeyword::Continue => {
                match &self.value {
                    Some(value) => {
                        value.process(None, context);
                        context.errors.add_and_none(CompilationError::unexpected_expression(value))
                    },
                    None => {
                        match context.get_scope_depth(ScopeKind::Loop) {
                            Some(depth) => {
                                match &self.keyword.value {
                                    ActionKeyword::Break => Some(vasm![VI::jump(depth + 1)]),
                                    ActionKeyword::Continue => Some(vasm![VI::jump(depth)]),
                                    _ => unreachable!()
                                }
                            },
                            None => {
                                context.errors.add_and_none(CompilationError::unexpected_keyword(&self.keyword, &format!("{}", &self.keyword.value)))
                            }
                        }
                    }
                }
            },
        }
    }
}