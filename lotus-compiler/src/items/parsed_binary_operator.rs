use std::borrow::Cow;

use parsable::{ItemLocation, parsable};
use colored::*;
use crate::{items::Identifier, program::{BuiltinInterface, CompilationError, IS_NONE_METHOD_NAME, NONE_METHOD_NAME, ProgramContext, Type, VariableInfo, VariableKind, Vasm}, wat};

#[parsable]
#[derive(Default, Clone)]
pub struct ParsedBinaryOperator {
    pub token: ParsedBinaryOperatorToken
}

#[parsable(impl_display=true, name="binary operator")]
#[derive(PartialEq, Clone, Copy)]
pub enum ParsedBinaryOperatorToken {
    Plus = "+",
    Minus = "-",
    Mult = "*",
    Div = "/",
    Mod = "%",
    Shl = "<<",
    Shr = ">>",
    Xor = "^",
    DoubleAnd = "&&",
    DoubleOr = "||",
    SingleAnd = "&",
    SingleOr = "|",
    Eq = "==",
    Ne = "!=",
    Ge = ">=",
    Gt = ">",
    Le = "<=",
    Lt = "<",
    // Range = "..",
}

impl ParsedBinaryOperator {
    pub fn new(value: ParsedBinaryOperatorToken, location: &ItemLocation) -> Self {
        let mut result = Self::default();

        result.token = value;
        result.location = location.clone();

        result
    }

    pub fn is_selective_operator(&self) -> bool {
        match &self.token {
            ParsedBinaryOperatorToken::DoubleAnd | ParsedBinaryOperatorToken::DoubleOr => true,
            _ => false
        }
    }

    pub fn get_priority(&self) -> usize {
        match &self.token {
            ParsedBinaryOperatorToken::Mult | ParsedBinaryOperatorToken::Div | ParsedBinaryOperatorToken::Mod => 1,
            ParsedBinaryOperatorToken::Plus | ParsedBinaryOperatorToken::Minus => 2,
            ParsedBinaryOperatorToken::Shl | ParsedBinaryOperatorToken::Shr => 3,
            ParsedBinaryOperatorToken::SingleAnd => 4,
            ParsedBinaryOperatorToken::SingleOr => 5,
            ParsedBinaryOperatorToken::Xor => 6,
            ParsedBinaryOperatorToken::Eq | ParsedBinaryOperatorToken::Ne | ParsedBinaryOperatorToken::Ge | ParsedBinaryOperatorToken::Gt | ParsedBinaryOperatorToken::Le | ParsedBinaryOperatorToken::Lt => 7,
            ParsedBinaryOperatorToken::DoubleAnd => 8,
            ParsedBinaryOperatorToken::DoubleOr => 9,
            // BinaryOperator::Range => 10
        }
    }

    pub fn get_short_circuit_vasm(&self, context: &ProgramContext) -> Option<Vasm> {
        match &self.token {
            ParsedBinaryOperatorToken::DoubleAnd | ParsedBinaryOperatorToken::DoubleOr => {
                let tmp_var = VariableInfo::tmp("tmp", context.bool_type());
                let mut result = context.vasm()
                    .declare_variable(&tmp_var)
                    .tee_tmp_var(&tmp_var)
                    .get_tmp_var(&tmp_var)
                    .chain(|vasm| {
                        match &self.token {
                            ParsedBinaryOperatorToken::DoubleAnd => vasm.eqz(),
                            _ => vasm
                        }
                    })
                    .jump_if_from_stack(0)
                    .set_type(context.bool_type());

                Some(result)
            }
            _ => None
        }
    }

    pub fn get_type_hint<'a>(&self, left_type: &'a Type, context: &ProgramContext) -> Option<Cow<'a, Type>> {
        match &self.token {
            ParsedBinaryOperatorToken::Plus => Some(Cow::Borrowed(left_type)),
            ParsedBinaryOperatorToken::Minus => Some(Cow::Borrowed(left_type)),
            ParsedBinaryOperatorToken::Mult => Some(Cow::Borrowed(left_type)),
            ParsedBinaryOperatorToken::Div => Some(Cow::Borrowed(left_type)),
            ParsedBinaryOperatorToken::Mod => Some(Cow::Borrowed(left_type)),
            ParsedBinaryOperatorToken::Shl => Some(Cow::Owned(context.int_type())),
            ParsedBinaryOperatorToken::Shr => Some(Cow::Owned(context.int_type())),
            ParsedBinaryOperatorToken::Xor => Some(Cow::Borrowed(left_type)),
            ParsedBinaryOperatorToken::DoubleAnd => None,
            ParsedBinaryOperatorToken::DoubleOr => None,
            ParsedBinaryOperatorToken::SingleAnd => Some(Cow::Borrowed(left_type)),
            ParsedBinaryOperatorToken::SingleOr => Some(Cow::Borrowed(left_type)),
            ParsedBinaryOperatorToken::Eq => Some(Cow::Borrowed(left_type)),
            ParsedBinaryOperatorToken::Ne => Some(Cow::Borrowed(left_type)),
            ParsedBinaryOperatorToken::Ge => Some(Cow::Borrowed(left_type)),
            ParsedBinaryOperatorToken::Gt => Some(Cow::Borrowed(left_type)),
            ParsedBinaryOperatorToken::Le => Some(Cow::Borrowed(left_type)),
            ParsedBinaryOperatorToken::Lt => Some(Cow::Borrowed(left_type)),
        }
    }

    pub fn process(&self, left_vasm: Vasm, right_vasm: Vasm, right_location: &ItemLocation, context: &mut ProgramContext) -> Option<Vasm> {
        let operator_kind = match &self.token {
            ParsedBinaryOperatorToken::Plus => OperatorKind::BuiltinInterface(BuiltinInterface::Add),
            ParsedBinaryOperatorToken::Minus => OperatorKind::BuiltinInterface(BuiltinInterface::Sub),
            ParsedBinaryOperatorToken::Mult => OperatorKind::BuiltinInterface(BuiltinInterface::Mul),
            ParsedBinaryOperatorToken::Div => OperatorKind::BuiltinInterface(BuiltinInterface::Div),
            ParsedBinaryOperatorToken::Mod => OperatorKind::BuiltinInterface(BuiltinInterface::Mod),
            ParsedBinaryOperatorToken::Shl => OperatorKind::BuiltinInterface(BuiltinInterface::Shl),
            ParsedBinaryOperatorToken::Shr => OperatorKind::BuiltinInterface(BuiltinInterface::Shr),
            ParsedBinaryOperatorToken::Xor => OperatorKind::BuiltinInterface(BuiltinInterface::Xor),
            ParsedBinaryOperatorToken::DoubleAnd => OperatorKind::Selective(SelectiveOperator::And),
            ParsedBinaryOperatorToken::DoubleOr => OperatorKind::Selective(SelectiveOperator::Or),
            ParsedBinaryOperatorToken::SingleAnd => OperatorKind::BuiltinInterface(BuiltinInterface::And),
            ParsedBinaryOperatorToken::SingleOr => OperatorKind::BuiltinInterface(BuiltinInterface::Or),
            ParsedBinaryOperatorToken::Eq => OperatorKind::Equality(EqualityOperator::Equal),
            ParsedBinaryOperatorToken::Ne => OperatorKind::Equality(EqualityOperator::NotEqual),
            ParsedBinaryOperatorToken::Ge => OperatorKind::BuiltinInterface(BuiltinInterface::Ge),
            ParsedBinaryOperatorToken::Gt => OperatorKind::BuiltinInterface(BuiltinInterface::Gt),
            ParsedBinaryOperatorToken::Le => OperatorKind::BuiltinInterface(BuiltinInterface::Le),
            ParsedBinaryOperatorToken::Lt => OperatorKind::BuiltinInterface(BuiltinInterface::Lt),
            // BinaryOperator::Range => OperatorKind::BuiltinInterface(BuiltinInterface::Range),
        };

        match operator_kind {
            OperatorKind::Equality(kind) => {
                let method_name = match kind {
                    EqualityOperator::Equal => "__eq",
                    EqualityOperator::NotEqual => "__ne",
                };

                match right_vasm.ty.is_assignable_to(&left_vasm.ty) || left_vasm.ty.is_assignable_to(&right_vasm.ty) {
                    true => {
                        let operator_vasm = context.vasm()
                            .call_regular_method(&left_vasm.ty, method_name, &[], vec![], context)
                            .set_type(context.bool_type());

                        let result = context.vasm()
                            .append(left_vasm)
                            .append(right_vasm)
                            .append(operator_vasm);

                        Some(result)
                    },
                    false => {
                        context.errors.type_mismatch(right_location, &left_vasm.ty, &right_vasm.ty);
                        None
                    },
                }
            },
            OperatorKind::Selective(kind) => {
                match kind {
                    SelectiveOperator::And => {
                        let return_type = right_vasm.ty.clone();
                        let condition = context.vasm().call_regular_method(&left_vasm.ty, IS_NONE_METHOD_NAME, &[], vec![], context);
                        let then_branch = context.vasm().call_static_method(&right_vasm.ty, NONE_METHOD_NAME, &[], vec![], context);
                        let else_branch = right_vasm;
                        let mut result = context.vasm()
                            .append(left_vasm)
                            .if_then_else(Some(&return_type), condition, then_branch, else_branch)
                            .set_type(return_type);

                        Some(result)
                    },
                    SelectiveOperator::Or => match left_vasm.ty.get_common_type(&right_vasm.ty) {
                        Some(return_type) => {
                            let return_type = return_type.clone();
                            let tmp_var = VariableInfo::tmp("tmp", return_type.clone());
                            let condition = context.vasm()
                                .tee_tmp_var(&tmp_var)
                                .call_regular_method(&return_type, IS_NONE_METHOD_NAME, &[], vec![], context);
                            let then_branch = right_vasm;
                            let else_branch = context.vasm()
                                .get_tmp_var(&tmp_var);

                            let result = context.vasm()
                                .declare_variable(tmp_var)
                                .append(left_vasm)
                                .if_then_else(Some(&return_type), condition, then_branch, else_branch)
                                .set_type(return_type);

                            Some(result)
                        },
                        None => {
                            context.errors.type_mismatch(right_location, &left_vasm.ty, &right_vasm.ty);
                            None
                        },
                    },
                }
            },
            OperatorKind::BuiltinInterface(required_interface) => match left_vasm.ty.call_builtin_interface(self, required_interface, &[(&right_vasm.ty, right_location)], context, || format!("")) {
                Some(operator_vasm) => Some(
                    context.vasm()
                        .append(left_vasm)
                        .append(right_vasm)
                        .append(operator_vasm)
                ),
                None => None,
            },
        }
    }
}

enum OperatorKind {
    Equality(EqualityOperator),
    Selective(SelectiveOperator),
    BuiltinInterface(BuiltinInterface)
}

enum EqualityOperator {
    Equal,
    NotEqual
}

enum SelectiveOperator {
    And,
    Or
}

impl PartialEq for SelectiveOperator {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

impl Default for ParsedBinaryOperatorToken {
    fn default() -> Self {
        Self::Plus
    }
}