use std::ops::Deref;
use parsable::DataLocation;
use super::CompilationError;

#[derive(Debug)]
pub struct CompilationErrorList {
    errors: Vec<CompilationError>,
    enabled: bool
}

impl CompilationErrorList {
    pub fn new() -> Self {
        Self {
            errors: vec![],
            enabled: true,
        }
    }

    pub fn add(&mut self, error: CompilationError) {
        if self.enabled {
            self.errors.push(error)
        }
    }

    pub fn add_and_none<T>(&mut self, error: CompilationError) -> Option<T>{
        self.add(error);

        None
    }

    pub fn add_generic(&mut self, location: &DataLocation, error: String) {
        self.add(CompilationError::generic(location, error))
    }

    pub fn add_generic_unlocated(&mut self, error: String) {
        self.add(CompilationError::generic_unlocated(error))
    }

    pub fn add_generic_and_none<T>(&mut self, location: &DataLocation, error: String) -> Option<T> {
        self.add_and_none(CompilationError::generic(location, error))
    }

    pub fn set_enabled(&mut self, value: bool) {
        self.enabled = value;
    }

    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn consume(self) -> Vec<CompilationError> {
        self.errors
    }
}

impl Default for CompilationErrorList {
    fn default() -> Self {
        Self::new()
    }
}