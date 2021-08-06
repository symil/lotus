use parsable::parsable;

use super::{BinaryOperator, Operand, Type};

#[parsable]
pub struct Operation {
    pub first: Operand,
    pub others: Vec<(BinaryOperator, Operand)>,
    #[parsable(prefix="as")]
    pub as_type: Option<Type>
}