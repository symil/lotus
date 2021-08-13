use parsable::parsable;

use super::{Identifier, FullType};

#[parsable]
pub struct FunctionArgument {
    pub name: Identifier,
    #[parsable(prefix=":")]
    pub ty: FullType,
}