use std::{collections::HashSet, ops::Deref};
use colored::*;
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

    pub fn from_parse_error(error: ParseError, file_name: &'static str, namespace_name: &'static str, ) -> Self {
        let mut expected_set = HashSet::new();
        let mut expected_list = vec![];

        for token in error.expected {
            if expected_set.insert(token.clone()) {
                expected_list.push(token);
            }
        }

        let string = match expected_list.len() {
            1 => format!("expected {}", expected_list[0]),
            _ => format!("expected ({})", expected_list.join(" | ")),
        };

        Self {
            location: Some(DataLocation {
                start: 0,
                end: 0,
                namespace_name,
                file_name,
                line: error.line,
                column: error.column
            }),
            error: string
        }
    }

    pub fn to_string(&self) -> String {
        let error_string = format!("{} {}", "error:".red().bold(), self.error);
        let location_string = match &self.location {
            Some(location) => format!("{}:{}:{}: ", location.file_name, location.line, location.column),
            None => String::new(),
        };

        format!("{}{}", location_string.bold(), error_string)
    }
}