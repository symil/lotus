use parsable::parsable;

use super::{Expression, VarPath};

#[parsable]
pub struct Assignment {
    pub lvalue: VarPath,
    #[parsable(prefix="=")]
    pub rvalue: Option<Expression>
}