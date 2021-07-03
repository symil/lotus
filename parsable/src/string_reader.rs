use std::{collections::HashMap};

use regex::Regex;

use super::parse_error::ParseError;

pub struct StringReader<'a> {
    regexes: HashMap<&'static str, Regex>,
    string: &'a str,
    index: usize,
    error: Option<ParseError>
}

impl<'a> StringReader<'a> {
    pub fn new(string: &'a str) -> Self {
        Self {
            regexes: HashMap::new(),
            string,
            index: 0,
            error: None
        }
    }

    pub fn set_error<T>(&mut self) {
        if self.index >= self.error.as_ref().map_or(0usize, |err| err.index) {
            self.error = Some(ParseError::new::<T>(self.index));
        }
    }

    pub fn get_error(&self) -> Option<ParseError> {
        self.error.clone()
    }

    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn set_index(&mut self, index: usize) {
        self.index = index;
    }

    pub fn advance(&mut self, length: usize) -> Option<&'a str> {
        match length {
            0 => None,
            _ => {
                let start = self.index;
                let end = self.index + length;

                self.index = end;
                Some(&self.string[start..end])
            }
        }
    }

    pub fn as_str(&self) -> &str {
        &self.string[self.index..]
    }

    pub fn as_char(&self) -> char {
        match self.as_str().as_bytes().first() {
            Some(byte) => *byte as char,
            None => 0 as char
        }
    }

    pub fn at(&self, index: usize) -> char {
        match self.string.as_bytes().get(self.index + index) {
            Some(byte) => *byte as char,
            None => 0 as char,
        }
    }

    pub fn eat_spaces(&mut self) {
        while is_space(self.as_char()) {
            self.index += 1;
        }
    }

    pub fn read_function<F : Fn(&str) -> usize>(&mut self, f: F) -> Option<&'a str> {
        self.advance(f(self.as_str()))
    }

    pub fn read_string(&mut self, string: &'static str) -> Option<&'a str> {
        let length = match self.as_str().starts_with(string) {
            true => string.len(),
            false => 0
        };

        self.advance(length)
    }

    pub fn read_regex(&mut self, pattern: &'static str) -> Option<&'a str> {
        let regex = match self.regexes.get(pattern) {
            Some(value) => value,
            None => {
                let re = Regex::new(&format!("^{}", pattern)).unwrap();
                self.regexes.insert(pattern, re);
                self.regexes.get(pattern).unwrap()
            }
        };

        let length = match regex.find(self.as_str()) {
            Some(m) => m.end(),
            None => 0
        };

        self.advance(length)
    }
}

fn is_space(c: char) -> bool {
    match c {
        ' ' | '\n' | '\t' => true,
        _ => false
    }
}