use parsable::{DataLocation, parsable};
use crate::{generation::{Wat}, items::Identifier, program::{ARRAY_CONCAT_FUNC_NAME, BuiltinInterface, ProgramContext, STRING_CONCAT_FUNC_NAME, STRING_EQUAL_FUNC_NAME, Type, VI, VariableInfo, VariableKind, Vasm}, wat};

#[parsable]
#[derive(Default)]
pub struct BinaryOperatorWrapper {
    pub value: BinaryOperator
}

#[parsable(impl_display=true)]
#[derive(PartialEq)]
pub enum BinaryOperator {
    Plus = "+",
    Minus = "-",
    Mult = "*",
    Div = "/",
    Mod = "%",
    Shl = "<<",
    Shr = ">>",
    DoubleAnd = "&&",
    DoubleOr = "||",
    And = "&",
    Or = "|",
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
            BinaryOperator::And => 4,
            BinaryOperator::Or => 5,
            BinaryOperator::Eq | BinaryOperator::Ne | BinaryOperator::Ge | BinaryOperator::Gt | BinaryOperator::Le | BinaryOperator::Lt => 6,
            BinaryOperator::DoubleAnd => 7,
            BinaryOperator::DoubleOr => 8,
        }
    }

    pub fn get_short_circuit_vasm(&self, context: &ProgramContext) -> Option<Vasm> {
        match &self.value {
            BinaryOperator::DoubleAnd | BinaryOperator::DoubleOr => {
                let tmp_var = VariableInfo::new(Identifier::unique("tmp", self), context.bool_type(), VariableKind::Local);
                let mut content = vec![
                    VI::tee(&tmp_var),
                    VI::get(&tmp_var),
                ];

                if &self.value == &BinaryOperator::DoubleAnd {
                    content.push(VI::Raw(wat!["i32.eqz"]));
                }

                content.push(VI::jump(0, None));

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
            BinaryOperator::DoubleAnd => BuiltinInterface::And,
            BinaryOperator::DoubleOr => BuiltinInterface::Or,
            BinaryOperator::And => BuiltinInterface::And,
            BinaryOperator::Or => BuiltinInterface::Or,
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