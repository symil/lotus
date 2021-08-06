use parsable::parsable;
use super::{VarRef};

#[parsable]
pub struct MethodCondition {
    pub left: VarRef,
    #[parsable(prefix="=")]
    pub right: Option<VarRef>
}