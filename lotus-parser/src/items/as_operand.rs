use parsable::parsable;

use super::{Operand, FullType};

#[parsable]
pub struct AsOperand {
    pub main: Operand,
    #[parsable(prefix="as")]
    pub as_type: FullType
}