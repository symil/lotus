use std::fmt::{Debug, Formatter, Result};

#[derive(Clone)]
pub struct Logger {
    log_function: fn(&str)
}

impl Logger {
    pub fn new(log_function: fn(&str)) -> Self {
        Self {
            log_function
        }
    }

    pub fn log(&self, value: &str) {
        (self.log_function)(value);
    }

    pub fn log_value<T : Debug>(&self, value: &T) {
        (self.log_function)(&format!("{:?}", value));
    }
}

impl Debug for Logger {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str("[Logger]")
    }
}