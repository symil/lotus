use std::{collections::HashMap, rc::Rc};
use parsable::parsable;
use crate::{items::{AssignmentOperatorValue, BinaryOperator, BinaryOperatorWrapper}, program::{AccessType, CompilationError, ProgramContext, Type, Vasm}, wat};
use super::{AssignmentOperator, Expression, Identifier, VarPath, VarRef};

#[parsable]
pub struct Assignment {
    pub lvalue: VarPath,
    pub rvalue: Option<(AssignmentOperator, Expression)>,
}

impl Assignment {
    pub fn collected_instancied_type_names(&self, list: &mut Vec<Identifier>, context: &mut ProgramContext) {
        match &self.rvalue {
            Some(rvalue) => {},
            None => self.lvalue.collected_instancied_type_names(list, context),
        }
    }

    pub fn process(&self, type_hint: Option<&Type>, context: &mut ProgramContext) -> Option<Vasm> {
        let mut result = None;

        if let Some((equal_token, rvalue)) = &self.rvalue {
            if let Some(mut left_vasm) = self.lvalue.process(None, AccessType::Set(&equal_token), context) {
                if let Some(right_vasm) = rvalue.process(Some(&left_vasm.ty), context) {
                    if right_vasm.ty.is_assignable_to(&left_vasm.ty) {
                        let assigned_vasm = match &equal_token.value {
                            AssignmentOperatorValue::Equal => right_vasm,
                            _ => {
                                let associated_binary_operator = match &equal_token.value {
                                    AssignmentOperatorValue::Equal => unreachable!(),
                                    AssignmentOperatorValue::PlusEqual => BinaryOperator::Plus,
                                    AssignmentOperatorValue::MinusEqual => BinaryOperator::Minus,
                                    AssignmentOperatorValue::MultEqual => BinaryOperator::Mult,
                                    AssignmentOperatorValue::DivEqual => BinaryOperator::Div,
                                    AssignmentOperatorValue::ModEqual => BinaryOperator::Mod,
                                    AssignmentOperatorValue::ShlEqual => BinaryOperator::Shl,
                                    AssignmentOperatorValue::ShrEqual => BinaryOperator::Shr,
                                    AssignmentOperatorValue::XorEqual => BinaryOperator::Xor,
                                    AssignmentOperatorValue::DoubleAndEqual => BinaryOperator::DoubleAnd,
                                    AssignmentOperatorValue::DoubleOrEqual => BinaryOperator::DoubleOr,
                                    AssignmentOperatorValue::SingleAndEqual => BinaryOperator::SingleAnd,
                                    AssignmentOperatorValue::SingleOrEqual => BinaryOperator::SingleOr,
                                };
                                let wrapper = BinaryOperatorWrapper::new(associated_binary_operator, &equal_token.location);

                                match self.lvalue.process(None, AccessType::Get, context) {
                                    Some(left_rvalue_vasm) => match wrapper.process(left_rvalue_vasm, right_vasm, rvalue, context) {
                                        Some(vasm) => vasm,
                                        None => context.vasm()
                                    },
                                    None => context.vasm()
                                }
                            }
                        };

                        left_vasm.replace_placeholder(&equal_token, &Rc::new(assigned_vasm));
                        left_vasm.ty = context.void_type();
                        
                        result = Some(left_vasm);
                    } else {
                        context.errors.type_mismatch(rvalue, &left_vasm.ty, &right_vasm.ty);
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