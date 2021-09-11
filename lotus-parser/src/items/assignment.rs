use std::collections::HashMap;

use parsable::parsable;
use crate::{generation::Wat, items::{AssignmentOperator, BinaryOperator, BinaryOperatorWrapper}, program::{AccessType, ProgramContext, Type, TypeOld, IrFragment}, wat};
use super::{AssignmentOperatorWrapper, Expression, VarPath};

#[parsable]
pub struct Assignment {
    pub lvalue: VarPath,
    pub rvalue: Option<(AssignmentOperatorWrapper, Expression)>
}

impl Assignment {
    pub fn process(&self, context: &mut ProgramContext) -> Option<IrFragment> {
        let mut result = None;

        if let Some((equal_token, rvalue)) = &self.rvalue {
            let left_wasm_opt = self.lvalue.process(AccessType::Set(&equal_token), context);
            let right_wasm_opt = rvalue.process(context);

            if let Some(left_wasm) = left_wasm_opt {
                if let Some(right_wasm) = right_wasm_opt {
                    if !right_wasm.ty.is_assignable() {
                        context.errors.add(rvalue, format!("cannot assign type `{}`", &right_wasm.ty));
                    } else if left_wasm.ty.is_assignable_to(&right_wasm.ty, context) {
                        let mut source = vec![];
                        let mut ok = true;

                        if equal_token.value != AssignmentOperator::Equal {
                            let associated_binary_operator = match &equal_token.value {
                                AssignmentOperator::Equal => unreachable!(),
                                AssignmentOperator::PlusEqual => BinaryOperator::Plus,
                                AssignmentOperator::MinusEqual => BinaryOperator::Minus,
                                AssignmentOperator::MultEqual => BinaryOperator::Mult,
                                AssignmentOperator::DivEqual => BinaryOperator::Div,
                                AssignmentOperator::ModEqual => BinaryOperator::Mod,
                                AssignmentOperator::ShlEqual => BinaryOperator::Shl,
                                AssignmentOperator::ShrEqual => BinaryOperator::Shr,
                                AssignmentOperator::DoubleAndEqual => BinaryOperator::DoubleAnd,
                                AssignmentOperator::DoubleOrEqual => BinaryOperator::DoubleOr,
                                AssignmentOperator::AndEqual => BinaryOperator::And,
                                AssignmentOperator::OrEqual => BinaryOperator::Or,
                            };
                            let wrapper = BinaryOperatorWrapper::new(associated_binary_operator, &equal_token.location);

                            if let Some(left_rvalue_wasm) = self.lvalue.process(AccessType::Get, context) {
                                if let Some(operator_wasm) = wrapper.process(&left_rvalue_wasm.ty, &right_wasm.ty, context) {
                                    source.push(left_rvalue_wasm);
                                    source.push(right_wasm);
                                    source.push(operator_wasm);
                                } else {
                                    context.errors.add(equal_token, format!("operator `{}` cannot be applied to type `{}`", &equal_token.value, &left_rvalue_wasm.ty));
                                    ok = false;
                                }
                            }
                        } else {
                            source.push(right_wasm);
                        }
                        
                        source.push(left_wasm);

                        if ok {
                            result = Some(IrFragment::merge(Type::Void, source));
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
                    source.push(IrFragment::new(Type::Void, wat!["drop"], vec![]));
                }

                result = Some(IrFragment::merge(Type::Void, source));
            }
        }

        result
    }
}