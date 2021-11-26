use std::{collections::HashMap};
use parsable::parsable;
use crate::{items::convert_to_bool, program::{CompilationError, ProgramContext, ScopeKind, Type, VI, Vasm}, vasm, wat};
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
                match context.get_current_function_return_type() {
                    Some(return_type) => {
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
                    None => {
                        context.errors.add_and_none(CompilationError::unexpected_keyword(&self.keyword, &format!("{}", &self.keyword.value)))
                    }
                }
            },
            ActionKeyword::Check => {
                match context.get_current_function_return_type() {
                    Some(return_type) => {
                        match &self.value {
                            Some(value) => {
                                match value.process(None, context) {
                                    Some(vasm) => match convert_to_bool(value, vasm, context) {
                                        Some(bool_vasm) => Some(vasm![
                                            VI::if_then_else(None, bool_vasm, vasm![], vasm![
                                                VI::return_value(vasm![VI::none(&return_type, context)])
                                            ])
                                        ]),
                                        None => None,
                                    },
                                    None => None,
                                }
                            },
                            None => {
                                context.errors.add_and_none(CompilationError::expected_expression(&self.keyword.location.get_end()))
                            },
                        }
                    },
                    None => None,
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