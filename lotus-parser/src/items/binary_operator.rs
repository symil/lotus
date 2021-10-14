use parsable::{DataLocation, parsable};
use crate::{items::Identifier, program::{BuiltinInterface, ProgramContext, Type, VI, VariableInfo, VariableKind, Vasm}, wat};

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
}

impl BinaryOperatorWrapper {
    pub fn new(value: BinaryOperator, location: &DataLocation) -> Self {
        let mut result = Self::default();

        result.value = value;
        result.location = location.clone();

        result
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
        }
    }

    pub fn get_short_circuit_vasm(&self, context: &ProgramContext) -> Option<Vasm> {
        match &self.value {
            BinaryOperator::DoubleAnd | BinaryOperator::DoubleOr => {
                let tmp_var = VariableInfo::new(Identifier::unique("tmp", self), context.bool_type(), VariableKind::Local);
                let mut content = vec![
                    VI::tee_from_stack(&tmp_var),
                    VI::get(&tmp_var),
                ];

                if &self.value == &BinaryOperator::DoubleAnd {
                    content.push(VI::Raw(wat!["i32.eqz"]));
                }

                content.push(VI::jump_if_from_stack(0));

                Some(Vasm::new(context.bool_type(), vec![tmp_var], content))
            }
            _ => None
        }
    }

    pub fn process(&self, left_type: &Type, right_type: &Type, context: &mut ProgramContext) -> Option<Vasm> {
        let required_interface = match &self.value {
            BinaryOperator::Plus => BuiltinInterface::Add,
            BinaryOperator::Minus => BuiltinInterface::Sub,
            BinaryOperator::Mult => BuiltinInterface::Mul,
            BinaryOperator::Div => BuiltinInterface::Div,
            BinaryOperator::Mod => BuiltinInterface::Mod,
            BinaryOperator::Shl => BuiltinInterface::Shl,
            BinaryOperator::Shr => BuiltinInterface::Shr,
            BinaryOperator::Xor => BuiltinInterface::Xor,
            BinaryOperator::DoubleAnd => BuiltinInterface::And,
            BinaryOperator::DoubleOr => BuiltinInterface::Or,
            BinaryOperator::SingleAnd => BuiltinInterface::And,
            BinaryOperator::SingleOr => BuiltinInterface::Or,
            BinaryOperator::Eq => BuiltinInterface::Eq,
            BinaryOperator::Ne => BuiltinInterface::Ne,
            BinaryOperator::Ge => BuiltinInterface::Ge,
            BinaryOperator::Gt => BuiltinInterface::Gt,
            BinaryOperator::Le => BuiltinInterface::Le,
            BinaryOperator::Lt => BuiltinInterface::Lt,
        };

        context.call_builtin_interface(self, required_interface, left_type, &[right_type], || format!("`{}` right operand", &self.value))
    }
}

impl Default for BinaryOperator {
    fn default() -> Self {
        Self::Plus
    }
}