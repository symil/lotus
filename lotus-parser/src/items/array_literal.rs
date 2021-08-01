use parsable::parsable;

use super::Expression;

#[parsable]
pub struct ArrayLiteral {
    #[parsable(brackets="[]", separator=",")]
    pub items: Vec<Expression>
}