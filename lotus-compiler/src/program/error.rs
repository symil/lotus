use std::{collections::HashSet, ops::Deref};
use colored::*;
use parsable::{DataLocation, Parsable, ParseError};

#[derive(Debug, Clone)]
pub struct Error {
    pub location: Option<DataLocation>,
    pub error: String,
    pub details: Vec<String>
}

impl Error {
    pub fn unlocated<S : Deref<Target=str>>(error: S) -> Self {
        Self {
            location: None,
            error: error.to_string(),
            details: vec![],
        }
    }

    pub fn located<S : Deref<Target=str>>(location: &DataLocation, error: S) -> Self {
        Self {
            location: Some(location.clone()),
            error: error.to_string(),
            details: vec![],
        }
    }

    pub fn located_detailed<S : Deref<Target=str>, T : Deref<Target=str>>(location: &DataLocation, error: S, details: Vec<T>) -> Self {
        Self {
            location: Some(location.clone()),
            error: error.to_string(),
            details: details.into_iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn from_parse_error(error: ParseError) -> Self {
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
                file_name: error.file_name,
                file_namespace: error.file_namespace,
                file_content: error.file_content,
                line: error.line,
                column: error.column
            }),
            error: string,
            details: vec![],
        }
    }

    pub fn to_string(&self) -> String {
        let error_string = format!("{} {}", "error:".red().bold(), self.error);
        let location_string = match &self.location {
            Some(location) => format!("{}:{}:{}: ", location.file_name, location.line, location.column),
            None => String::new(),
        };

        let mut result = format!("{}{}", location_string.bold(), error_string);

        for detail in &self.details {
            result.push_str(&format!("\n  - {}", detail));
        }

        result
    }
}