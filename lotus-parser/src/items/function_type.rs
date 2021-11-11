use parsable::parsable;
use super::ParsedType;

#[parsable]
pub struct FunctionType {
    #[parsable(brackets="()", separator=",")]
    pub arguments: Vec<ParsedType>,
    #[parsable(prefix="()")]
    pub return_value: Option<Box<ParsedType>>
}