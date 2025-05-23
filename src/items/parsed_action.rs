use std::{collections::HashMap};
use parsable::parsable;
use crate::{items::convert_to_bool, program::{BuiltinType, CompilationError, ProgramContext, ScopeKind, Type, Vasm, EVENT_OPTIONS_VAR_NAME}, wat};
use super::{ParsedActionKeyword, ParsedActionKeywordToken, ParsedExpression};

#[parsable]
pub struct ParsedAction {
    pub keyword: ParsedActionKeyword,
    pub expression: Option<ParsedExpression>
}

impl ParsedAction {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Vasm> {
        let keyword = format!("{}", &self.keyword.token);

        match &self.keyword.token {
            ParsedActionKeywordToken::Return => {
                match context.get_current_function_return_type() {
                    Some(return_type) => {
                        match return_type.is_void() {
                            true => {
                                match &self.expression {
                                    Some(expr) => {
                                        match expr.process(None, context) {
                                            Some(vasm) => match vasm.ty.is_void() {
                                                true => Some(vasm),
                                                false => {
                                                    context.errors.type_mismatch(expr, &context.void_type(), &vasm.ty);
                                                    None
                                                }
                                            },
                                            None => None,
                                        }
                                    },
                                    None => {
                                        Some(
                                            context.vasm()
                                                .return_value(context.vasm().set_void(context))
                                                .set_type(context.void_type())
                                        )
                                    },
                                }
                            },
                            false => {
                                match &self.expression {
                                    Some(expr) => match expr.process(Some(&return_type), context) {
                                        Some(vasm) => match vasm.ty.is_assignable_to(&return_type) {
                                            true => {
                                                Some(
                                                    context.vasm()
                                                        .return_value(vasm)
                                                        .set_type(context.void_type())
                                                )
                                            },
                                            false => {
                                                context.errors.type_mismatch(expr, &return_type, &vasm.ty);
                                                None
                                            },
                                        },
                                        None => None,
                                    },
                                    None => {
                                        context.errors.type_mismatch(self, &return_type, &context.void_type());
                                        None
                                    }
                                }
                            },
                        }
                    },
                    None => {
                        context.errors.unexpected_keyword(&self.keyword, &keyword);
                        None
                    }
                }
            },
            ParsedActionKeywordToken::Check => {
                match context.get_current_function_return_type() {
                    Some(return_type) => {
                        match &self.expression {
                            Some(value) => {
                                match value.process(None, context) {
                                    Some(vasm) => match convert_to_bool(value, vasm, context) {
                                        Some(bool_vasm) => Some(context.vasm()
                                            .if_then_else(None, bool_vasm, context.vasm(), context.vasm()
                                                .return_value(context.vasm().none(&return_type, context))
                                            )
                                            .set_type(context.void_type())
                                        ),
                                        None => None,
                                    },
                                    None => None,
                                }
                            },
                            None => {
                                context.errors.expected_expression(&self.keyword.location.get_end());
                                None
                            },
                        }
                    },
                    None => None,
                }
            },
            ParsedActionKeywordToken::Break | ParsedActionKeywordToken::Continue => {
                match &self.expression {
                    Some(expr) => {
                        expr.process(None, context);
                        context.errors.unexpected_expression(expr);
                        None
                    },
                    None => {
                        match context.get_scope_depth(ScopeKind::Loop) {
                            Some(depth) => {
                                match &self.keyword.token {
                                    ParsedActionKeywordToken::Break => Some(context.vasm()
                                        .jump(depth + 1)
                                        .set_type(context.void_type())
                                    ),
                                    ParsedActionKeywordToken::Continue => Some(context.vasm()
                                        .jump(depth)
                                        .set_type(context.void_type())
                                    ),
                                    _ => unreachable!()
                                }
                            },
                            None => {
                                context.errors.unexpected_keyword(&self.keyword, &keyword);
                                None
                            }
                        }
                    }
                }
            },
            ParsedActionKeywordToken::Intercept | ParsedActionKeywordToken::Yield => {
                match context.get_current_function() {
                    Some(function_wrapped) => match function_wrapped.borrow().is_event_callback() {
                        true => {
                            let output_var = function_wrapped.borrow().argument_variables.iter().find(|var_info| var_info.name().as_str() == EVENT_OPTIONS_VAR_NAME).unwrap().clone();
                            let event_output_type = context.get_builtin_type(BuiltinType::EventOptions, vec![]);

                            match &self.keyword.token {
                                ParsedActionKeywordToken::Intercept => match &self.expression {
                                    Some(expr) => {
                                        context.errors.unexpected_expression(expr);
                                        None
                                    },
                                    None => {
                                        let intercepted_field_info = event_output_type.get_field("intercepted").unwrap();

                                        Some(context.vasm()
                                            .get_var(&output_var, None)
                                            .set_field(&intercepted_field_info.ty, intercepted_field_info.offset, None, context.vasm().int(1i32))
                                            .return_value(context.vasm())
                                            .set_type(context.void_type())
                                        )
                                    },
                                },
                                ParsedActionKeywordToken::Yield => match &self.expression {
                                    Some(expr) => match expr.process(None, context) {
                                        Some(vasm) => {
                                            let yielded_field_info = event_output_type.get_field("yielded").unwrap();

                                            Some(context.vasm()
                                                .get_var(&output_var, None)
                                                .get_field(&yielded_field_info.ty, yielded_field_info.offset, None)
                                                .call_regular_method(&yielded_field_info.ty, "push", &[], vec![vasm], context)
                                                .drop(&yielded_field_info.ty)
                                                .set_type(context.void_type())
                                            )
                                        },
                                        None => None,
                                    },
                                    None => {
                                        context.errors.expected_expression(&self.keyword.location.get_end());
                                        None
                                    },
                                },
                                _ => unreachable!()
                            }
                        },
                        false => {
                            context.errors.unexpected_keyword(&self.keyword, &keyword);
                            None
                        },
                    },
                    None => {
                        context.errors.unexpected_keyword(&self.keyword, &keyword);
                        None
                    },
                }
            }
        }
    }
}