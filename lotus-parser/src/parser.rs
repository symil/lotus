use std::fs;

use crate::{items::{expr::{Expr}}};
use parsable::*;

pub struct LotusParser {
    pub current_file_id: usize
}

impl LotusParser {
    pub fn new() -> Self {
        Self {
            current_file_id: 0,
        }
    }

    pub fn parse_root(&mut self, file_path: &str) {
        let unparsed_file = fs::read_to_string(file_path).expect("cannot read file");

        let result = Expr::parse_string(&unparsed_file);

        match result {
            Ok(value) => { dbg!(value); },
            Err(error) => { println!("{}", error.to_string()) },
        };
    }
}