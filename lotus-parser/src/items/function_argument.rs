use parsable::parsable;

use super::{Identifier, FullType};

#[parsable]
pub struct FunctionArgument {
    pub ty: FullType,
    pub name: Identifier,
}