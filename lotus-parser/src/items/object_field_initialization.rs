use parsable::parsable;
use crate::program::{ProgramContext, Vasm};

use super::{Expression, Identifier};

#[parsable]
pub struct ObjectFieldInitialization {
    pub name: Identifier,
    #[parsable(prefix=":")]
    pub value: Option<Expression>
}