use std::{collections::HashMap};

use line_col::LineColLookup;
use regex::Regex;

use super::parse_error::ParseError;

pub struct StringReader<'a> {
    regexes: HashMap<&'static str, Regex>,
    string: &'a str,
    index: usize,
    error_index: usize,
    expected: Vec<String>
}

impl<'a> StringReader<'a> {
    pub fn new(string: &'a str) -> Self {
        Self {
            regexes: HashMap::new(),
            string,
            index: 0,
            error_index: 0,
            expected: vec![]
        }
    }

    pub fn set_expected_token(&mut self, expected: String) {
        if self.index == self.error_index {
            self.expected.push(expected);
        } else if self.index > self.error_index {
            self.expected = vec![expected];
            self.error_index = self.index;
        }
    }

    pub fn get_error(&self) -> ParseError {
        let mut error_index = self.error_index;
        let mut backtracked = false;

        while error_index > 0 && is_space(self.string.as_bytes()[error_index - 1] as char) {
            error_index -= 1;
            backtracked = true;
        }

        if backtracked {
            while error_index < self.string.len() && is_inline_space(self.string.as_bytes()[error_index] as char) {
                error_index += 1;
            }
        }

        let (line, col) = LineColLookup::new(&self.string).get(error_index);
        let expected = self.expected.clone();

        ParseError { line, col, expected }
    }

    pub fn is_finished(&self) -> bool {
        self.index == self.string.len()
    }

    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn get_index_backtracked(&self) -> usize {
        let mut index = self.index;

        while index > 0 && is_space(self.string.as_bytes()[index - 1] as char) {
            index -= 1;
        }

        index
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
            false => return None
        };

        // TODO: handle this at compile-time
        if is_string_alphanum(string) && is_alphanum(self.at(length)) {
            return None;
        }

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

fn is_inline_space(c: char) -> bool {
    match c {
        ' ' | '\t' => true,
        _ => false
    }
}

fn is_string_alphanum(string: &str) -> bool {
    for byte in string.as_bytes() {
        if !is_alphanum(*byte as char) {
            return false;
        }
    }

    true
}

fn is_alphanum(c: char) -> bool {
    (c >= '0' && c <= '9') || (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
}