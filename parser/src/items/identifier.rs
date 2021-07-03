use std::ops::{Deref, DerefMut};

use lotus_parsable::*;

pub struct Identifier {
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

#[parsable]
#[derive(Debug)]
pub struct Number {
    pub value: f32
}