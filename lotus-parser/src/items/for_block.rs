use parsable::parsable;

use super::{Expression, Identifier, Statement};

#[parsable]
pub struct ForBlock {
    #[parsable(prefix="for")]
    pub var_name: Identifier,
    #[parsable(prefix="in")]
    pub array_expression: Expression,
    #[parsable(brackets="{}")]
    pub statements: Vec<Statement>
}