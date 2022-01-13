use std::{collections::HashMap, rc::Rc};
use parsable::parsable;
use crate::{items::{ParsedAssignmentOperatorToken, ParsedBinaryOperatorToken, ParsedBinaryOperator}, program::{AccessType, CompilationError, ProgramContext, Type, Vasm}, wat};
use super::{ParsedAssignmentOperator, ParsedExpression, Identifier, ParsedVarPath, ParsedVarRef};

#[parsable]
pub struct ParsedAssignment {
    pub lvalue: ParsedVarPath,
    pub rvalue: Option<(ParsedAssignmentOperator, ParsedExpression)>,
}

impl ParsedAssignment {
    pub fn collected_instancied_type_names(&self, list: &mut Vec<String>, context: &mut ProgramContext) {
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
                        let assigned_vasm = match &equal_token.token {
                            ParsedAssignmentOperatorToken::Equal => right_vasm,
                            _ => {
                                let associated_binary_operator = match &equal_token.token {
                                    ParsedAssignmentOperatorToken::Equal => unreachable!(),
                                    ParsedAssignmentOperatorToken::PlusEqual => ParsedBinaryOperatorToken::Plus,
                                    ParsedAssignmentOperatorToken::MinusEqual => ParsedBinaryOperatorToken::Minus,
                                    ParsedAssignmentOperatorToken::MultEqual => ParsedBinaryOperatorToken::Mult,
                                    ParsedAssignmentOperatorToken::DivEqual => ParsedBinaryOperatorToken::Div,
                                    ParsedAssignmentOperatorToken::ModEqual => ParsedBinaryOperatorToken::Mod,
                                    ParsedAssignmentOperatorToken::ShlEqual => ParsedBinaryOperatorToken::Shl,
                                    ParsedAssignmentOperatorToken::ShrEqual => ParsedBinaryOperatorToken::Shr,
                                    ParsedAssignmentOperatorToken::XorEqual => ParsedBinaryOperatorToken::Xor,
                                    ParsedAssignmentOperatorToken::DoubleAndEqual => ParsedBinaryOperatorToken::DoubleAnd,
                                    ParsedAssignmentOperatorToken::DoubleOrEqual => ParsedBinaryOperatorToken::DoubleOr,
                                    ParsedAssignmentOperatorToken::SingleAndEqual => ParsedBinaryOperatorToken::SingleAnd,
                                    ParsedAssignmentOperatorToken::SingleOrEqual => ParsedBinaryOperatorToken::SingleOr,
                                };
                                let wrapper = ParsedBinaryOperator::new(associated_binary_operator, &equal_token.location);

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