use parsable::parsable;

use super::{Identifier, FullType};

#[parsable]
pub struct FunctionArgument {
    #[parsable(suffix=":")]
    pub name: Option<Identifier>,
    pub ty: FullType,
}