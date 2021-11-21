use std::collections::HashMap;
use parsable::parsable;
use crate::{items::{AssignmentOperator, BinaryOperator, BinaryOperatorWrapper}, program::{AccessType, ProgramContext, Type, VI, Vasm}, vasm, wat};
use super::{AssignmentOperatorWrapper, Expression, Identifier, VarPath, VarRef};

#[parsable]
pub struct Assignment {
    pub lvalue: VarPath,
    pub rvalue: Option<(AssignmentOperatorWrapper, Expression)>,
}

impl Assignment {
    pub fn collected_instancied_type_names(&self, list: &mut Vec<Identifier>) {
        match &self.rvalue {
            Some(rvalue) => {},
            None => self.lvalue.collected_instancied_type_names(list),
        }
    }

    pub fn process(&self, type_hint: Option<&Type>, context: &mut ProgramContext) -> Option<Vasm> {
        let mut result = None;

        if let Some((equal_token, rvalue)) = &self.rvalue {
            if let Some(left_vasm) = self.lvalue.process(None, AccessType::Set(&equal_token), context) {
                if let Some(right_vasm) = rvalue.process(Some(&left_vasm.ty), context) {
                    if right_vasm.ty.is_assignable_to(&left_vasm.ty) {
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
                                AssignmentOperator::XorEqual => BinaryOperator::Xor,
                                AssignmentOperator::DoubleAndEqual => BinaryOperator::DoubleAnd,
                                AssignmentOperator::DoubleOrEqual => BinaryOperator::DoubleOr,
                                AssignmentOperator::SingleAndEqual => BinaryOperator::SingleAnd,
                                AssignmentOperator::SingleOrEqual => BinaryOperator::SingleOr,
                            };
                            let wrapper = BinaryOperatorWrapper::new(associated_binary_operator, &equal_token.location);

                            if let Some(left_rvalue_vasm) = self.lvalue.process(None, AccessType::Get, context) {
                                if let Some(vasm) = wrapper.process(left_rvalue_vasm, right_vasm, rvalue, context) {
                                    source.push(vasm);
                                } else {
                                    ok = false;
                                }
                            }
                        } else {
                            source.push(right_vasm);
                        }
                        
                        source.push(left_vasm);

                        if ok {
                            result = Some(Vasm::merge_with_type(context.void_type(), source));
                        }
                    } else {
                        context.errors.add(rvalue, format!("expected `{}`, got `{}`", &left_vasm.ty, &right_vasm.ty));
                    }
                }
            }
        } else {
            if let Some(vasm) = self.lvalue.process(type_hint, AccessType::Get, context) {
                result = Some(vasm);
            }
        }

        result
    }
}