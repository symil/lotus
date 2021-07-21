use parsable::{DataLocation, Parsable};

#[derive(Debug, Clone)]
pub struct Error {
    pub location: DataLocation,
    pub error: String
}

impl Error {
    pub fn new(location: &DataLocation, error: String) -> Self {
        Self {
            location: location.clone(),
            error
        }
    }

    pub fn from<T : Parsable>(data: &T, error: &str) -> Self {
        Self {
            location: data.get_location().clone(),
            error: error.to_string()
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}:{}:{} => {}", self.location.file_name, self.location.line, self.location.column, self.error)
    }
}

pub struct ErrorList {
    pub errors: Vec<Error>
}

impl ErrorList {
    pub fn new() -> Self {
        Self { errors: vec![] }
    }

    pub fn add<T : Parsable>(&mut self, data: &T, error: &str) {
        self.errors.push(Error::from(data, error));
    }
}