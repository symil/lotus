use std::collections::HashMap;

use parsable::parsable;
use crate::{generation::Wat, items::{AssignmentToken, BinaryOperatorToken}, program::{AccessType, ProgramContext, TypeOld, Wasm}, wat};
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
                    if !right_wasm.ty.is_assignable() {
                        context.errors.add(rvalue, format!("cannot assign type `{}`", &right_wasm.ty));
                    } else if left_wasm.ty.is_assignable_to(&right_wasm.ty, context, &mut HashMap::new()) {
                        let mut source = vec![];
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
                                    source.push(left_rvalue_wasm);
                                    source.push(right_wasm);
                                    source.push(operator_wasm);
                                } else {
                                    context.errors.add(equal_token, format!("operator `{}` cannot be applied to type `{}`", &equal_token.token, &left_rvalue_wasm.ty));
                                    ok = false;
                                }
                            }
                        } else {
                            source.push(right_wasm);
                        }
                        
                        source.push(left_wasm);

                        if ok {
                            result = Some(Wasm::merge(TypeOld::Void, source));
                        }
                    } else {
                        context.errors.add(rvalue, format!("expected `{}`, got `{}`", &left_wasm.ty, &right_wasm.ty));
                    }
                }
            }
        } else {
            if let Some(wasm) = self.lvalue.process(AccessType::Get, context) {
                let is_void = wasm.ty.is_void();
                let mut source = vec![wasm];

                if !is_void {
                    source.push(Wasm::new(TypeOld::Void, wat!["drop"], vec![]));
                }

                result = Some(Wasm::merge(TypeOld::Void, source));
            }
        }

        result
    }
}