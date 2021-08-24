use std::collections::HashMap;

use parsable::parsable;
use crate::{generation::Wat, items::{AssignmentToken, BinaryOperatorToken}, program::{AccessType, ProgramContext, Wasm}};
use super::{AssignmentOperator, Expression, VarPath};

#[parsable]
pub struct Assignment {
    pub lvalue: VarPath,
    pub rvalue: Option<(AssignmentOperator, Expression)>
}

impl Assignment {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Wasm> {
        let mut result = None;

        if let Some((equal_token, rvalue)) = &self.rvalue {
            let left_wasm_opt = self.lvalue.process(AccessType::Set(&equal_token), context);
            let right_wasm_opt = rvalue.process(context);

            if let Some(left_wasm) = left_wasm_opt {
                if let Some(right_wasm) = right_wasm_opt {
                    if left_wasm.ty.is_assignable(&right_wasm.ty, context, &mut HashMap::new()) {
                        let mut wat = vec![];
                        let mut ok = true;

                        if equal_token.token != AssignmentToken::Equal {
                            let associated_binary_operator = match &equal_token.token {
                                AssignmentToken::Equal => unreachable!(),
                                AssignmentToken::PlusEqual => BinaryOperatorToken::Plus,
                                AssignmentToken::MinusEqual => BinaryOperatorToken::Minus,
                                AssignmentToken::MultEqual => BinaryOperatorToken::Mult,
                                AssignmentToken::DivEqual => BinaryOperatorToken::Div,
                                AssignmentToken::ModEqual => BinaryOperatorToken::Mod,
                                AssignmentToken::ShlEqual => BinaryOperatorToken::Shl,
                                AssignmentToken::ShrEqual => BinaryOperatorToken::Shr,
                                AssignmentToken::AndEqual => BinaryOperatorToken::And,
                                AssignmentToken::OrEqual => BinaryOperatorToken::Or,
                            };

                            if let Some(left_rvalue_wasm) = self.lvalue.process(AccessType::Get, context) {
                                if let Some(operator_wasm) = associated_binary_operator.process(&left_rvalue_wasm.ty, context) {
                                    wat.extend(left_rvalue_wasm.wat);
                                    wat.extend(right_wasm.wat);
                                    wat.extend(operator_wasm.wat);
                                } else {
                                    context.error(equal_token, format!("operator `{}` cannot be applied to type `{}`", &equal_token.token, &left_rvalue_wasm.ty));
                                    ok = false;
                                }
                            }
                        } else {
                            wat.extend(right_wasm.wat);
                        }

                        wat.extend(left_wasm.wat);

                        if ok {
                            result = Some(Wasm::untyped(wat, vec![]));
                        }
                    } else {
                        context.error(rvalue, format!("expected `{}`, got `{}`", &left_wasm.ty, &right_wasm.ty));
                    }
                }
            }
        } else {
            if let Some(wasm) = self.lvalue.process(AccessType::Get, context) {
                let mut wat = vec![];

                wat.extend(wasm.wat);

                if !wasm.ty.is_void() {
                    wat.push(Wat::inst("drop"));
                }

                result = Some(Wasm::untyped(wat, vec![]));
            }
        }

        result
    }
}