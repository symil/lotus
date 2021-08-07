use parsable::parsable;

use super::{Expression, VarPath};

#[parsable]
pub struct Assignment {
    pub lvalue: VarPath,
    pub rvalue: Option<(AssignmentToken, Expression)>
}

#[parsable]
pub enum AssignmentToken {
    Equal = "="
}