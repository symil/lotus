use std::{collections::hash_map::DefaultHasher, fmt::Display, hash::{Hash, Hasher}, ops::{Deref, DerefMut}};
use parsable::*;

static mut COUNTER : usize = 0;

#[parsable(name="identifier")]
#[derive(Default, Clone)]
pub struct Identifier {
    #[parsable(regex = r#"[a-zA-Z_][_\w\d]*"#)]
    pub value: String
}

impl Identifier {
    pub fn new(name: &str, location: &DataLocation) -> Self {
        let mut identifier = Identifier::default();

        identifier.value = name.to_string();
        identifier.location = location.clone();

        identifier
    }

    pub fn unique(name: &str) -> Self {
        let mut identifier = Identifier::default();
        let id = unsafe {
            COUNTER += 1;
            COUNTER
        };

        identifier.value = format!("{}_u{}", name.to_string(), id);

        identifier
    }

    pub fn unlocated(name: &str) -> Self {
        let mut identifier = Identifier::default();
        identifier.value = name.to_string();
        
        identifier
    }

    pub fn to_unique_string(&self) -> String {
        match self.location.is_empty() {
            true => self.to_string(),
            false => format!("{}_{}", self.as_str(), self.location.get_hash()),
        }
    }

    pub fn get_u32_hash(&self) -> u32 {
        let mut hasher = DefaultHasher::new();

        self.value.hash(&mut hasher);
        self.location.hash(&mut hasher);

        hasher.finish() as u32
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
        self.location.hash(state);
    }
}

impl PartialEq for Identifier {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.location == other.location
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