use std::ops::Deref;

use parsable::{DataLocation, Parsable};

#[derive(Debug, Clone)]
pub struct Error {
    pub location: Option<DataLocation>,
    pub error: String
}

impl Error {
    pub fn new<S : Deref<Target=str>>(error: S) -> Self {
        Self {
            location: None,
            error: error.to_string()
        }
    }

    pub fn from<T : Parsable, S : Deref<Target=str>>(data: &T, error: S) -> Self {
        Self {
            location: Some(data.get_location().clone()),
            error: error.to_string()
        }
    }

    pub fn to_string(&self) -> String {
        if let Some(location) = &self.location {
            format!("{}:{}:{} => {}", location.file_name, location.line, location.column, self.error)
        } else {
            self.error.clone()
        }
    }
}