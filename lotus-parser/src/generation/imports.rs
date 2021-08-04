use super::{Wat, ToWat, ToWatVec};

pub struct Imports;

impl Imports {
    pub fn new() -> Self {
        Self
    }

    pub fn get_header(&self) -> Vec<Wat> {
        vec![
            Wat::import_function("log", "i32", "log_i32", vec!["i32"], None)
        ]
    }
}