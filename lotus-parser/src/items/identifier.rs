use std::{fmt::Display, hash::Hash, ops::{Deref, DerefMut}};

use parsable::*;

#[parsable(name="identifier")]
#[derive(Default)]
pub struct Identifier {
    #[parsable(regex = r#"[a-zA-Z_][_\w\d]*"#)]
    pub value: String
}

impl Identifier {
    pub fn new<S : Deref<Target=str>>(name: S) -> Self {
        let mut identifier = Identifier::default();

        identifier.value = name.to_string();

        identifier
    }

    pub fn new_unique<L : Deref<Target=DataLocation>>(prefix: &str, location: &L) -> Self {
        Self::new(format!("{}_{:#01X}", prefix, location.start * 65536 + location.end))
    }

    pub fn to_unique_string(&self) -> String {
        Self::new_unique(self.as_str(), self).to_string()
    }

    pub fn is(&self, value: &str) -> bool {
        self.value == value
    }

    pub fn as_str(&self) -> &str {
        self.value.as_str()
    }

    pub fn to_string(&self) -> String {
        self.value.clone()
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