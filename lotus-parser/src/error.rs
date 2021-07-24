use std::ops::Deref;

use parsable::{DataLocation, Parsable};

#[derive(Debug, Clone)]
pub struct Error {
    pub location: DataLocation,
    pub error: String
}

impl Error {
    pub fn new<T : Parsable, S : Deref<Target=str>>(data: &T, error: S) -> Self {
        Self {
            location: data.get_location().clone(),
            error: error.to_string()
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}:{}:{} => {}", self.location.file_name, self.location.line, self.location.column, self.error)
    }
}