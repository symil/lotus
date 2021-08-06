use parsable::parsable;
use super::AnyType;

#[parsable]
pub struct FunctionType {
    #[parsable(brackets="()", separator=",")]
    pub arguments: Vec<AnyType>,
    #[parsable(prefix="()")]
    pub return_value: Option<Box<AnyType>>
}