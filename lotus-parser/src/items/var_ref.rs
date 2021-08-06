use parsable::parsable;

use super::{Identifier, VarRefPrefix};

#[parsable]
pub struct VarRef {
    pub prefix: Option<VarRefPrefix>,
    pub name: Identifier
}

impl VarRef {
    pub fn has_this_prefix(&self) -> bool {
        match self.prefix {
            Some(VarRefPrefix::This) => true,
            _ => false
        }
    }

    pub fn has_payload_prefix(&self) -> bool {
        match self.prefix {
            Some(VarRefPrefix::Payload) => true,
            _ => false
        }
    }
}