use std::ops::Deref;
use parsable::DataLocation;
use super::Error;

#[derive(Debug, Default)]
pub struct ErrorList {
    errors: Vec<Error>
}

impl ErrorList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add<S : Deref<Target=str>>(&self, location: &DataLocation, error: S) {
        self.errors.push(Error::located(location, error));
    }

    pub fn consume(self) -> Vec<Error> {
        self.errors
    }
}