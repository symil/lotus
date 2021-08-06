use parsable::parsable;

use super::{Expression, Operand};

#[parsable]
pub struct Assignment {
    pub lvalue: Operand,
    #[parsable(prefix="=")]
    pub rvalue: Option<Expression>
}