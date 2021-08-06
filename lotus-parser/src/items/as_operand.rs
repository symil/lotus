use parsable::parsable;

use super::{Operand, AnyType};

#[parsable]
pub struct AsOperand {
    pub main: Operand,
    #[parsable(prefix="as")]
    pub as_type: AnyType
}