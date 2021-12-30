use std::{collections::{HashMap, HashSet}};
use regex::Regex;
use crate::{DataLocation};
use super::parse_error::ParseError;

pub struct StringReader {
    comment_token: &'static str,

    package_root_path: &'static str,
    file_path: &'static str,
    file_content: &'static str,

    index: usize,
    error_index: usize,
    expected: Vec<String>,
    markers: HashMap<&'static str, Vec<bool>>
}

pub struct ParseOptions<'a, 'b, 'c> {
    pub file_path: Option<&'a str>,
    pub package_root_path: Option<&'b str>,
    pub comment_start: Option<&'c str>
}

static mut INITIALIZED : bool = false;
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
    fn init() {
        unsafe {
            if !INITIALIZED {
                REGEXES = Some(HashMap::new());
                STRINGS = Some(HashSet::new());
                INITIALIZED = true;
            }
        }
    }

    pub fn clear_all_static_strings() {
        unsafe {
            STRINGS = Some(HashSet::new());
        }
    }

    pub fn new(content: String, options: ParseOptions) -> Self {
        Self::init();

        Self {
            comment_token: get_str(options.comment_start.unwrap_or("").to_string()),
            package_root_path: get_str(options.package_root_path.unwrap_or("").to_string()),
            file_path: get_str(options.file_path.unwrap_or("").to_string()),
            file_content: get_str(content),
            index: 0,
            error_index: 0,
            expected: vec![],
            markers: HashMap::new()
        }
    }

    pub fn get_file_path(&self) -> &'static str {
        self.file_path
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

        while error_index > 0 && is_space(self.file_content.as_bytes()[error_index - 1] as char) {
            error_index -= 1;
            backtracked = true;
        }

        if backtracked {
            while error_index < self.file_content.len() && is_inline_space(self.file_content.as_bytes()[error_index] as char) {
                error_index += 1;
            }
        }

        ParseError {
            package_root_path: self.package_root_path,
            file_path: self.file_path,
            file_content: self.file_content,
            index: error_index,
            expected: self.expected.clone(),
        }
    }

    pub fn is_finished(&self) -> bool {
        self.index == self.file_content.len()
    }

    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn get_index_backtracked(&self) -> usize {
        let mut index = self.index;

        // TODO: handle comments
        while index > 0 && is_space(self.file_content.as_bytes()[index - 1] as char) {
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
                Some(&self.file_content[start..end])
            }
        }
    }

    pub fn as_str(&self) -> &str {
        &self.file_content[self.index..]
    }

    pub fn as_char(&self) -> char {
        match self.as_str().as_bytes().first() {
            Some(byte) => *byte as char,
            None => 0 as char
        }
    }

    pub fn at(&self, index: usize) -> char {
        match self.file_content.as_bytes().get(self.index + index) {
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

                while self.as_char() != '\n' && self.index < self.file_content.len() {
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

    pub fn peek_regex(&mut self, pattern: &'static str) -> bool {
        let regex = get_regex(pattern);

        regex.find(self.as_str()).is_some()
    }

    pub fn get_data_location(&self, start: usize) -> DataLocation {
        DataLocation {
            package_root_path: self.package_root_path,
            file_path: self.file_path,
            file_content: self.file_content,
            start,
            end: self.get_index_backtracked(),
        }
    }

    pub fn get_marker_value(&self, name: &'static str) -> bool {
        match self.markers.get(name) {
            Some(list) => *list.last().unwrap_or(&false),
            None => false
        }
    }

    pub fn push_marker_value(&mut self, name: &'static str, value: bool) {
        if !self.markers.contains_key(name) {
            self.markers.insert(name, vec![]);
        }

        self.markers.get_mut(name).unwrap().push(value);
    }

    pub fn pop_marker_value(&mut self, name: &'static str) {
        if let Some(list) = self.markers.get_mut(name) {
            list.pop();
        }
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