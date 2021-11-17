use std::ops::Deref;
use parsable::DataLocation;
use super::Error;

#[derive(Debug)]
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

    pub fn add<S : Deref<Target=str>>(&mut self, location: &DataLocation, error: S) {
        if self.enabled {
            self.errors.push(Error::located(location, error));
        }
    }

    pub fn add_detailed<S : Deref<Target=str>, T : Deref<Target=str>>(&mut self, location: &DataLocation, error: S, details: Vec<T>) {
        if self.enabled {
            self.errors.push(Error::located_detailed(location, error, details));
        }
    }

    pub fn add_unlocated<S : Deref<Target=str>>(&mut self, error: S) {
        if self.enabled {
            self.errors.push(Error::unlocated(error));
        }
    }

    pub fn add_and_none<S : Deref<Target=str>, T>(&mut self, location: &DataLocation, error: S) -> Option<T>{
        if self.enabled {
            self.errors.push(Error::located(location, error));
        }

        None
    }

    pub fn add_and_false<S : Deref<Target=str>>(&mut self, location: &DataLocation, error: S) -> bool {
        if self.enabled {
            self.errors.push(Error::located(location, error));
        }

        false
    }

    pub fn set_enabled(&mut self, value: bool) {
        self.enabled = value;
    }

    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn consume(self) -> Vec<Error> {
        self.errors
    }
}

impl Default for ErrorList {
    fn default() -> Self {
        Self::new()
    }
}