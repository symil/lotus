use std::{collections::HashMap};
use parsable::parsable;
use crate::{items::convert_to_bool, program::{BuiltinType, CompilationError, ProgramContext, ScopeKind, Type, VI, Vasm}, vasm, wat};
use super::{ActionKeywordWrapper, ActionKeyword, Expression};

#[parsable]
pub struct Action {
    pub keyword: ActionKeywordWrapper,
    pub value: Option<Expression>
}

impl Action {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Vasm> {
        let keyword = format!("{}", &self.keyword.value);

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
                        context.errors.add_and_none(CompilationError::unexpected_keyword(&self.keyword, &keyword))
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
                                context.errors.add_and_none(CompilationError::unexpected_keyword(&self.keyword, &keyword))
                            }
                        }
                    }
                }
            },
            ActionKeyword::Intercept | ActionKeyword::Yield => {
                match context.get_current_function() {
                    Some(function_wrapped) => match function_wrapped.borrow().is_event_callback() {
                        true => {
                            let output_var = function_wrapped.borrow().argument_variables.iter().find(|var_info| var_info.name().as_str() == "__output").unwrap().clone();
                            let event_output_type = context.get_builtin_type(BuiltinType::EventOutput, vec![]);

                            match &self.keyword.value {
                                ActionKeyword::Intercept => {
                                    let intercepted_field_info = event_output_type.get_field("intercepted").unwrap();

                                    Some(vasm![
                                        VI::get_var(&output_var, None),
                                        VI::set_field(&intercepted_field_info.ty, intercepted_field_info.offset, vasm![ VI::int(1i32) ]),
                                        VI::return_value(vasm![])
                                    ])
                                },
                                ActionKeyword::Yield => todo!(),
                                _ => unreachable!()
                            }
                        },
                        false => {
                            context.errors.add_and_none(CompilationError::unexpected_keyword(&self.keyword, &keyword))
                        },
                    },
                    None => {
                        context.errors.add_and_none(CompilationError::unexpected_keyword(&self.keyword, &keyword))
                    },
                }
            }
        }
    }
}