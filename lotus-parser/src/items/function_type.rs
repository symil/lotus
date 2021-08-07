use parsable::parsable;
use super::FullType;

#[parsable]
pub struct FunctionType {
    #[parsable(brackets="()", separator=",")]
    pub arguments: Vec<FullType>,
    #[parsable(prefix="()")]
    pub return_value: Option<Box<FullType>>
}