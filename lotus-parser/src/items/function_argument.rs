use parsable::parsable;

use super::{Identifier, AnyType};

#[parsable]
pub struct FunctionArgument {
    pub name: Identifier,
    #[parsable(prefix=":")]
    pub type_: AnyType
}