use parsable::parsable;

use super::Variable;

#[parsable]
pub struct MethodCondition {
    pub left: Variable,
    #[parsable(prefix="=")]
    pub right: Option<Variable>
}