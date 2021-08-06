use parsable::parsable;

use super::{Operand, Type};

#[parsable]
pub struct AsOperand {
    pub main: Operand,
    #[parsable(prefix="as")]
    pub as_type: Type
}