use std::ops::Deref;
use parsable::DataLocation;
use super::Error;

#[derive(Debug, Default)]
pub struct ErrorList {
    errors: Vec<Error>,
    enabled: bool
}

impl ErrorList {
    pub fn new() -> Self {
        Self {
            errors: vec![],
            enabled: true,
        }
    }

    pub fn add<S : Deref<Target=str>>(&self, location: &DataLocation, error: S) {
        if self.enabled {
            self.errors.push(Error::located(location, error));
        }
    }

    pub fn set_enabled(&mut self, value: bool) {
        self.enabled = value;
    }

    pub fn consume(self) -> Vec<Error> {
        self.errors
    }
}