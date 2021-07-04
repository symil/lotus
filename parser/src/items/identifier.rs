use std::ops::{Deref, DerefMut};

use lotus_parsable::*;

#[parsable]
#[derive(Debug)]
pub struct Identifier {
    #[parsable(regex = r"[a-zA-Z_][_\w\d]*")]
    pub value: String
}

impl Deref for Identifier {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DerefMut for Identifier {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }    
}