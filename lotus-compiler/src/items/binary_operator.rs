use parsable::{DataLocation, parsable};
use colored::*;
use crate::{items::Identifier, program::{BuiltinInterface, CompilationError, IS_NONE_METHOD_NAME, NONE_METHOD_NAME, ProgramContext, Type, VariableInfo, VariableKind, Vasm}, wat};

#[parsable]
#[derive(Default, Clone)]
pub struct BinaryOperatorWrapper {
    pub value: BinaryOperator
}

#[parsable(impl_display=true, name="binary operator")]
#[derive(PartialEq, Clone, Copy)]
pub enum BinaryOperator {
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

impl BinaryOperatorWrapper {
    pub fn new(value: BinaryOperator, location: &DataLocation) -> Self {
        let mut result = Self::default();

        result.value = value;
        result.location = location.clone();

        result
    }

    pub fn is_selective_operator(&self) -> bool {
        match &self.value {
            BinaryOperator::DoubleAnd | BinaryOperator::DoubleOr => true,
            _ => false
        }
    }

    pub fn get_priority(&self) -> usize {
        match &self.value {
            BinaryOperator::Mult | BinaryOperator::Div | BinaryOperator::Mod => 1,
            BinaryOperator::Plus | BinaryOperator::Minus => 2,
            BinaryOperator::Shl | BinaryOperator::Shr => 3,
            BinaryOperator::SingleAnd => 4,
            BinaryOperator::SingleOr => 5,
            BinaryOperator::Xor => 6,
            BinaryOperator::Eq | BinaryOperator::Ne | BinaryOperator::Ge | BinaryOperator::Gt | BinaryOperator::Le | BinaryOperator::Lt => 7,
            BinaryOperator::DoubleAnd => 8,
            BinaryOperator::DoubleOr => 9,
            // BinaryOperator::Range => 10
        }
    }

    pub fn get_short_circuit_vasm(&self, context: &ProgramContext) -> Option<Vasm> {
        match &self.value {
            BinaryOperator::DoubleAnd | BinaryOperator::DoubleOr => {
                let tmp_var = VariableInfo::tmp("tmp", context.bool_type());
                let mut result = context.vasm()
                    .declare_variable(&tmp_var)
                    .tee_tmp_var(&tmp_var)
                    .get_tmp_var(&tmp_var)
                    .chain(|vasm| {
                        match &self.value {
                            BinaryOperator::DoubleAnd => vasm.eqz(),
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

    pub fn process(&self, left_vasm: Vasm, right_vasm: Vasm, right_location: &DataLocation, context: &mut ProgramContext) -> Option<Vasm> {
        let operator_kind = match &self.value {
            BinaryOperator::Plus => OperatorKind::BuiltinInterface(BuiltinInterface::Add),
            BinaryOperator::Minus => OperatorKind::BuiltinInterface(BuiltinInterface::Sub),
            BinaryOperator::Mult => OperatorKind::BuiltinInterface(BuiltinInterface::Mul),
            BinaryOperator::Div => OperatorKind::BuiltinInterface(BuiltinInterface::Div),
            BinaryOperator::Mod => OperatorKind::BuiltinInterface(BuiltinInterface::Mod),
            BinaryOperator::Shl => OperatorKind::BuiltinInterface(BuiltinInterface::Shl),
            BinaryOperator::Shr => OperatorKind::BuiltinInterface(BuiltinInterface::Shr),
            BinaryOperator::Xor => OperatorKind::BuiltinInterface(BuiltinInterface::Xor),
            BinaryOperator::DoubleAnd => OperatorKind::Selective(SelectiveOperator::And),
            BinaryOperator::DoubleOr => OperatorKind::Selective(SelectiveOperator::Or),
            BinaryOperator::SingleAnd => OperatorKind::BuiltinInterface(BuiltinInterface::And),
            BinaryOperator::SingleOr => OperatorKind::BuiltinInterface(BuiltinInterface::Or),
            BinaryOperator::Eq => OperatorKind::Equality(EqualityOperator::Equal),
            BinaryOperator::Ne => OperatorKind::Equality(EqualityOperator::NotEqual),
            BinaryOperator::Ge => OperatorKind::BuiltinInterface(BuiltinInterface::Ge),
            BinaryOperator::Gt => OperatorKind::BuiltinInterface(BuiltinInterface::Gt),
            BinaryOperator::Le => OperatorKind::BuiltinInterface(BuiltinInterface::Le),
            BinaryOperator::Lt => OperatorKind::BuiltinInterface(BuiltinInterface::Lt),
            // BinaryOperator::Range => OperatorKind::BuiltinInterface(BuiltinInterface::Range),
        };

        match operator_kind {
            OperatorKind::Equality(kind) => {
                let method_name = match kind {
                    EqualityOperator::Equal => "eq",
                    EqualityOperator::NotEqual => "ne",
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

impl Default for BinaryOperator {
    fn default() -> Self {
        Self::Plus
    }
}