use parsable::parsable;

use super::{Expression, Statement};

#[parsable]
pub struct Branch {
    pub condition: Expression,
    #[parsable(brackets="{}")]
    pub statements: Vec<Statement>
}