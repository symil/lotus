use std::ops::{Deref, DerefMut};

item! {
    struct Identifier {
        value: String
    }

    entry => Identifier {
        value: entry.as_str().to_string()
    }
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