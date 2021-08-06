use parsable::parsable;

use super::{FunctionArgument, AnyType};

#[parsable]
pub struct FunctionSignature {
    #[parsable(brackets="()", separator=",")]
    pub arguments: Vec<FunctionArgument>,
    #[parsable(prefix="->")]
    pub return_type: Option<AnyType>,
}