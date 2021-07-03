use std::fs;

use crate::{items::{identifier::Number, type_qualifier::TypeQualifier}};
use lotus_parsable::*;

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

        let result = TypeQualifier::parse_string(&unparsed_file);

        match result {
            Ok(value) => { dbg!(value); },
            Err(error) => { dbg!(error); },
        };

        // let file = PestParser::parse(Rule::file, &unparsed_file)
        //     .expect("unsuccessful parse") // unwrap the parse result
        //     .next().unwrap(); // get and unwrap the `file` rule; never fails
        
        // let parsed_file = LotusFile::from(file);

        // dbg!(parsed_file);
    }
}