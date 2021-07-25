use std::{fmt::Display, hash::Hash, ops::{Deref, DerefMut}};

use parsable::*;

#[parsable(name="identifier")]
#[derive(Default)]
pub struct Identifier {
    #[parsable(regex = r#"[a-zA-Z_][_\w\d]*"#)]
    pub value: String
}

impl Identifier {
    pub fn is(&self, value: &str) -> bool {
        self.value == value
    }

    pub fn as_str(&self) -> &str {
        self.value.as_str()
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

impl Hash for Identifier {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl PartialEq for Identifier {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Identifier {
    
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}