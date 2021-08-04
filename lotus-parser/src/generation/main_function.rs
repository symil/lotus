use crate::wat;
use super::{Wat, ToWat};

pub struct MainFunction;

impl MainFunction {
    pub fn new() -> Self {
        Self
    }

    pub fn get_header(&self) -> Vec<Wat> {
        vec![
            Wat::function("main", Some("_start"), vec![], None, vec![
                wat!["call", "$log_i32", wat!["i32.const", 23]]
            ])
        ]
    }
}