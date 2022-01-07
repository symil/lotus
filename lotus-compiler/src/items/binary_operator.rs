use std::borrow::Cow;

use parsable::{DataLocation, parsable};
use colored::*;
use crate::{items::Identifier, program::{BuiltinInterface, CompilationError, IS_NONE_METHOD_NAME, NONE_METHOD_NAME, ProgramContext, Type, VariableInfo, VariableKind, Vasm}, wat};

#[parsable]
#[derive(Default, Clone)]
pub struct BinaryOperator {
    pub value: BinaryOperatorValue
}

#[parsable(impl_display=true, name="binary operator")]
#[derive(PartialEq, Clone, Copy)]
pub enum BinaryOperatorValue {
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

impl BinaryOperator {
    pub fn new(value: BinaryOperatorValue, location: &DataLocation) -> Self {
        let mut result = Self::default();

        result.value = value;
        result.location = location.clone();

        result
    }

    pub fn is_selective_operator(&self) -> bool {
        match &self.value {
            BinaryOperatorValue::DoubleAnd | BinaryOperatorValue::DoubleOr => true,
            _ => false
        }
    }

    pub fn get_priority(&self) -> usize {
        match &self.value {
            BinaryOperatorValue::Mult | BinaryOperatorValue::Div | BinaryOperatorValue::Mod => 1,
            BinaryOperatorValue::Plus | BinaryOperatorValue::Minus => 2,
            BinaryOperatorValue::Shl | BinaryOperatorValue::Shr => 3,
            BinaryOperatorValue::SingleAnd => 4,
            BinaryOperatorValue::SingleOr => 5,
            BinaryOperatorValue::Xor => 6,
            BinaryOperatorValue::Eq | BinaryOperatorValue::Ne | BinaryOperatorValue::Ge | BinaryOperatorValue::Gt | BinaryOperatorValue::Le | BinaryOperatorValue::Lt => 7,
            BinaryOperatorValue::DoubleAnd => 8,
            BinaryOperatorValue::DoubleOr => 9,
            // BinaryOperator::Range => 10
        }
    }

    pub fn get_short_circuit_vasm(&self, context: &ProgramContext) -> Option<Vasm> {
        match &self.value {
            BinaryOperatorValue::DoubleAnd | BinaryOperatorValue::DoubleOr => {
                let tmp_var = VariableInfo::tmp("tmp", context.bool_type());
                let mut result = context.vasm()
                    .declare_variable(&tmp_var)
                    .tee_tmp_var(&tmp_var)
                    .get_tmp_var(&tmp_var)
                    .chain(|vasm| {
                        match &self.value {
                            BinaryOperatorValue::DoubleAnd => vasm.eqz(),
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
        match &self.value {
            BinaryOperatorValue::Plus => Some(Cow::Borrowed(left_type)),
            BinaryOperatorValue::Minus => Some(Cow::Borrowed(left_type)),
            BinaryOperatorValue::Mult => Some(Cow::Borrowed(left_type)),
            BinaryOperatorValue::Div => Some(Cow::Borrowed(left_type)),
            BinaryOperatorValue::Mod => Some(Cow::Borrowed(left_type)),
            BinaryOperatorValue::Shl => Some(Cow::Owned(context.int_type())),
            BinaryOperatorValue::Shr => Some(Cow::Owned(context.int_type())),
            BinaryOperatorValue::Xor => Some(Cow::Borrowed(left_type)),
            BinaryOperatorValue::DoubleAnd => None,
            BinaryOperatorValue::DoubleOr => None,
            BinaryOperatorValue::SingleAnd => Some(Cow::Borrowed(left_type)),
            BinaryOperatorValue::SingleOr => Some(Cow::Borrowed(left_type)),
            BinaryOperatorValue::Eq => Some(Cow::Borrowed(left_type)),
            BinaryOperatorValue::Ne => Some(Cow::Borrowed(left_type)),
            BinaryOperatorValue::Ge => Some(Cow::Borrowed(left_type)),
            BinaryOperatorValue::Gt => Some(Cow::Borrowed(left_type)),
            BinaryOperatorValue::Le => Some(Cow::Borrowed(left_type)),
            BinaryOperatorValue::Lt => Some(Cow::Borrowed(left_type)),
        }
    }

    pub fn process(&self, left_vasm: Vasm, right_vasm: Vasm, right_location: &DataLocation, context: &mut ProgramContext) -> Option<Vasm> {
        let operator_kind = match &self.value {
            BinaryOperatorValue::Plus => OperatorKind::BuiltinInterface(BuiltinInterface::Add),
            BinaryOperatorValue::Minus => OperatorKind::BuiltinInterface(BuiltinInterface::Sub),
            BinaryOperatorValue::Mult => OperatorKind::BuiltinInterface(BuiltinInterface::Mul),
            BinaryOperatorValue::Div => OperatorKind::BuiltinInterface(BuiltinInterface::Div),
            BinaryOperatorValue::Mod => OperatorKind::BuiltinInterface(BuiltinInterface::Mod),
            BinaryOperatorValue::Shl => OperatorKind::BuiltinInterface(BuiltinInterface::Shl),
            BinaryOperatorValue::Shr => OperatorKind::BuiltinInterface(BuiltinInterface::Shr),
            BinaryOperatorValue::Xor => OperatorKind::BuiltinInterface(BuiltinInterface::Xor),
            BinaryOperatorValue::DoubleAnd => OperatorKind::Selective(SelectiveOperator::And),
            BinaryOperatorValue::DoubleOr => OperatorKind::Selective(SelectiveOperator::Or),
            BinaryOperatorValue::SingleAnd => OperatorKind::BuiltinInterface(BuiltinInterface::And),
            BinaryOperatorValue::SingleOr => OperatorKind::BuiltinInterface(BuiltinInterface::Or),
            BinaryOperatorValue::Eq => OperatorKind::Equality(EqualityOperator::Equal),
            BinaryOperatorValue::Ne => OperatorKind::Equality(EqualityOperator::NotEqual),
            BinaryOperatorValue::Ge => OperatorKind::BuiltinInterface(BuiltinInterface::Ge),
            BinaryOperatorValue::Gt => OperatorKind::BuiltinInterface(BuiltinInterface::Gt),
            BinaryOperatorValue::Le => OperatorKind::BuiltinInterface(BuiltinInterface::Le),
            BinaryOperatorValue::Lt => OperatorKind::BuiltinInterface(BuiltinInterface::Lt),
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

impl Default for BinaryOperatorValue {
    fn default() -> Self {
        Self::Plus
    }
}