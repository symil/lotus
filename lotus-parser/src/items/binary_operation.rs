use parsable::parsable;

use super::Operand;

#[parsable]
pub struct Operation {
    pub first: Operand,
    pub others: Vec<(BinaryOperator, Operand)>
}

#[parsable(impl_display=true)]
pub enum BinaryOperator {
    Plus = "+",
    Minus = "-",
    Mult = "*",
    Div = "/",
    Mod = "%",
    And = "&&",
    Or = "||",
    Eq = "==",
    Neq = "!=",
    Gte = ">=",
    Gt = ">",
    Lte = "<=",
    Lt = "<",
    Range = ".."
}