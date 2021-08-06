use parsable::parsable;

use super::{VarPathRoot, VarPathSegment};

#[parsable]
pub struct VarPath {
    pub root: VarPathRoot,
    pub path: Vec<VarPathSegment>
}