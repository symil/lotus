use std::ops::Deref;

use parsable::{DataLocation, Parsable, ParseError};

#[derive(Debug, Clone)]
pub struct Error {
    pub location: Option<DataLocation>,
    pub error: String
}

impl Error {
    pub fn unlocated<S : Deref<Target=str>>(error: S) -> Self {
        Self {
            location: None,
            error: error.to_string()
        }
    }

    pub fn located<S : Deref<Target=str>>(location: &DataLocation, error: S) -> Self {
        Self {
            location: Some(location.clone()),
            error: error.to_string()
        }
    }

    pub fn from_parse_error(error: ParseError, file_name: &'static str) -> Self {
        Self {
            location: Some(DataLocation {
                start: 0,
                end: 0,
                file_name,
                line: error.line,
                column: error.column
            }),
            error: format!("expected {}", error.expected.join(" | "))
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