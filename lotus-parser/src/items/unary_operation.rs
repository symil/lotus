use parsable::parsable;

use super::Operand;

#[parsable]
pub struct UnaryOperation {
    pub operator: UnaryOperator,
    pub operand: Operand
}

#[parsable(impl_display=true)]
pub enum UnaryOperator {
    Not = "!",
    Plus = "+",
    Minus = "-"
}