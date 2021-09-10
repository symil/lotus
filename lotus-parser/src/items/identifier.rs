use std::{fmt::Display, hash::Hash, ops::{Deref, DerefMut}};
use parsable::*;

static mut COUNTER : usize = 0;

#[parsable(name="identifier")]
#[derive(Default)]
pub struct Identifier {
    #[parsable(regex = r#"[a-zA-Z_][_\w\d]*"#)]
    pub value: String
}

impl Identifier {
    pub fn new<S : Deref<Target=str>, L : Deref<Target=DataLocation>>(name: S, location: &L) -> Self {
        let mut identifier = Identifier::default();

        identifier.value = name.to_string();
        identifier.location = location.deref().clone();

        identifier
    }

    pub fn unique<S : Deref<Target=str>>(name: S, location: &DataLocation) -> Self {
        let mut identifier = Identifier::default();
        let id = unsafe {
            COUNTER += 1;
            COUNTER
        };

        identifier.value = format!("{}{}", name.to_string(), id);
        identifier.location = location.clone();

        identifier
    }

    pub fn to_unique_string(&self) -> String {
        format!("{}_{}", self.as_str(), self.location.get_hash())
    }

    pub fn is(&self, value: &str) -> bool {
        self.value == value
    }

    pub fn as_str(&self) -> &str {
        self.value.as_str()
    }

    pub fn as_string(&self) -> &String {
        &self.value
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

impl AsRef<str> for Identifier {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<DataLocation> for Identifier {
    fn as_ref(&self) -> &DataLocation {
        &self.location
    }
}