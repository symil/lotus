use std::collections::HashMap;
use parsable::parsable;
use crate::{items::{AssignmentOperator, BinaryOperator, BinaryOperatorWrapper}, program::{AccessType, ProgramContext, Type, VI, Vasm}, wat};
use super::{AssignmentOperatorWrapper, Expression, VarPath};

#[parsable]
pub struct Assignment {
    pub lvalue: VarPath,
    pub rvalue: Option<(AssignmentOperatorWrapper, Expression)>
}

impl Assignment {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Vasm> {
        let mut result = None;

        if let Some((equal_token, rvalue)) = &self.rvalue {
            let left_vasm_opt = self.lvalue.process(AccessType::Set(&equal_token), context);
            let right_vasm_opt = rvalue.process(context);

            if let Some(left_vasm) = left_vasm_opt {
                if let Some(right_vasm) = right_vasm_opt {
                    if !right_vasm.ty.is_assignable() {
                        context.errors.add(rvalue, format!("cannot assign type `{}`", &right_vasm.ty));
                    } else if left_vasm.ty.is_assignable_to(&right_vasm.ty) {
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

                            if let Some(left_rvalue_vasm) = self.lvalue.process(AccessType::Get, context) {
                                if let Some(operator_vasm) = wrapper.process(&left_rvalue_vasm.ty, &right_vasm.ty, context) {
                                    source.push(left_rvalue_vasm);
                                    source.push(right_vasm);
                                    source.push(operator_vasm);
                                } else {
                                    context.errors.add(equal_token, format!("operator `{}` cannot be applied to type `{}`", &equal_token.value, &left_rvalue_vasm.ty));
                                    ok = false;
                                }
                            }
                        } else {
                            source.push(right_vasm);
                        }
                        
                        source.push(left_vasm);

                        if ok {
                            result = Some(Vasm::merge(source));
                        }
                    } else {
                        context.errors.add(rvalue, format!("expected `{}`, got `{}`", &left_vasm.ty, &right_vasm.ty));
                    }
                }
            }
        } else {
            if let Some(vasm) = self.lvalue.process(AccessType::Get, context) {
                let is_void = vasm.ty.is_void();
                let mut source = vec![vasm];

                if !is_void {
                    source.push(Vasm::new(Type::Void, vec![], vec![VI::Drop]));
                }

                result = Some(Vasm::merge(source));
            }
        }

        result
    }
}