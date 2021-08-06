use parsable::parsable;

use super::{Expression, Identifier, ObjectFieldInitialization};

#[parsable]
pub struct ObjectLiteral {
    pub type_name: Identifier,
    #[parsable(brackets="{}", separator=",")]
    pub fields: Vec<ObjectFieldInitialization>
}