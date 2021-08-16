use std::{collections::{HashMap, HashSet}};
use regex::Regex;
use crate::{DataLocation, line_col_lookup::LineColLookup};
use super::parse_error::ParseError;

pub struct StringReader {
    comment_token: &'static str,
    package_name: &'static str,
    file_name: &'static str,

    string: String,
    line_col: LineColLookup,
    index: usize,
    error_index: usize,
    expected: Vec<String>
}

static mut REGEXES : Option<HashMap<&'static str, Regex>> = None;
static mut STRINGS : Option<HashSet<String>> = None;

fn get_str(string: String) -> &'static str {
    unsafe {
        STRINGS.as_mut().unwrap().get_or_insert(string).as_str()
    }
}

fn get_regex(pattern: &'static str) -> &'static Regex {
    let regexes = unsafe { REGEXES.as_mut().unwrap() };

    if !regexes.contains_key(pattern) {
        regexes.insert(pattern, Regex::new(&format!("^({})", pattern)).unwrap());
    }

    regexes.get(pattern).unwrap()
}

impl StringReader {
    pub fn init() {
        unsafe {
            REGEXES = Some(HashMap::new());
            STRINGS = Some(HashSet::new());
        }
    }

    pub fn new(comment_token: &'static str) -> Self {
        Self {
            comment_token,
            package_name: "",
            file_name: "",
            string: String::new(),
            line_col: LineColLookup::new(""),
            index: 0,
            error_index: 0,
            expected: vec![]
        }
    }

    pub fn set_content(&mut self, package_name: String, file_name: String, file_content: String) {
        self.package_name = get_str(package_name);
        self.file_name = get_str(file_name);
        self.line_col = LineColLookup::new(&file_content);
        self.string = file_content;
        self.index = 0;
        self.error_index = 0;
        self.expected = vec![];
    }

    pub fn get_package_name(&self) -> &'static str {
        self.package_name
    }

    pub fn get_file_name(&self) -> &'static str {
        self.file_name
    }

    pub fn set_expected_token(&mut self, expected: Option<String>) {
        if let Some(expected) = expected {
            if self.index == self.error_index {
                self.expected.push(expected);
            } else if self.index > self.error_index {
                self.expected = vec![expected];
                self.error_index = self.index;
            }
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

        let (line, col) = self.line_col.get(error_index);
        let expected = self.expected.clone();

        ParseError { line, column: col, expected }
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

    pub fn advance(&mut self, length: usize) -> Option<&str> {
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
        let mut done = false;

        while !done {
            done = true;

            while is_space(self.as_char()) {
                self.index += 1;
            }

            if self.as_str().starts_with(self.comment_token) {
                done = false;

                while self.as_char() != '\n' {
                    self.index += 1;
                }
            }
        }
    }

    pub fn read_function<F : Fn(&str) -> usize>(&mut self, f: F) -> Option<&str> {
        self.advance(f(self.as_str()))
    }

    pub fn read_string(&mut self, string: &str) -> Option<&str> {
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

    pub fn read_regex(&mut self, pattern: &'static str) -> Option<&str> {
        let regex = get_regex(pattern);
        let length = match regex.find(self.as_str()) {
            Some(m) => m.end(),
            None => 0
        };

        self.advance(length)
    }

    pub fn get_data_location(&self, start: usize) -> DataLocation {
        let end = self.get_index_backtracked();
        let package_name = self.package_name;
        let file_name = self.file_name;
        let (line, column) = self.line_col.get(start);

        DataLocation { start, end, file_name, package_name, line, column }
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