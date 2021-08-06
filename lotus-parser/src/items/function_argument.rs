use parsable::parsable;

use super::{Identifier, Type};

#[parsable]
pub struct FunctionArgument {
    pub name: Identifier,
    #[parsable(prefix=":")]
    pub type_: Type
}