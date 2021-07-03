#[derive(Debug, Clone)]
pub struct ParseError {
    pub index: usize,
    pub expected_token: String
}

impl ParseError {
    pub fn new<T>(index: usize) -> Self {
        Self {
            index,
            expected_token: std::any::type_name::<T>().split("::").last().unwrap().to_string()
        }
    }
}