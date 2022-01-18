use std::{collections::{HashMap}, rc::Rc};
use regex::Regex;
use crate::{ItemLocation, file_info::FileInfo, Parsable};
use super::parse_error::ParseError;

pub struct StringReader {
    comment_token: &'static str,
    file: Rc<FileInfo>,
    index: usize,
    error_index: usize,
    expected: Vec<String>,
    markers: HashMap<&'static str, Vec<bool>>
}

pub struct ParseOptions {
    pub file_path: Option<String>,
    pub package_root_path: Option<String>,
    pub comment_start: Option<&'static str>
}

static mut REGEXES : Option<HashMap<&'static str, Regex>> = None;

fn get_regex(pattern: &'static str) -> &'static Regex {
    let regexes_opt = unsafe { &mut REGEXES };
    let regexes = regexes_opt.get_or_insert(HashMap::new());

    if !regexes.contains_key(pattern) {
        regexes.insert(pattern, Regex::new(&format!("^({})", pattern)).unwrap());
    }

    regexes.get(pattern).unwrap()
}

impl StringReader {
    pub fn new(content: String, options: ParseOptions) -> Self {
        Self {
            comment_token: options.comment_start.unwrap_or(""),
            file: Rc::new(FileInfo {
                path: options.file_path.unwrap_or_default(),
                content,
                package_root_path: options.package_root_path.unwrap_or_default(),
            }),
            index: 0,
            error_index: 0,
            expected: vec![],
            markers: HashMap::new()
        }
    }

    fn content(&self) -> &str {
        &self.file.content
    }

    pub fn set_expected_regex(&mut self, expected: &'static str) {
        self.set_expected_entity(format!("/{}/", expected));
    }

    pub fn set_expected_string(&mut self, expected: &'static str) {
        if !expected.is_empty() {
            self.set_expected_entity(format!("\"{}\"", expected));
        }
    }

    pub fn set_expected_item<T : Parsable>(&mut self) {
        self.set_expected_entity(T::get_item_name());
    }

    fn set_expected_entity(&mut self, string_to_display: String) {
        if self.index == self.error_index {
            self.expected.push(string_to_display);
        } else if self.index > self.error_index {
            self.expected = vec![string_to_display];
            self.error_index = self.index;
        }
    }

    pub fn get_error(&self) -> ParseError {
        let mut error_index = self.error_index;
        let mut backtracked = false;

        while error_index > 0 && is_space(self.content().as_bytes()[error_index - 1] as char) {
            error_index -= 1;
            backtracked = true;
        }

        if backtracked {
            while error_index < self.content().len() && is_inline_space(self.content().as_bytes()[error_index] as char) {
                error_index += 1;
            }
        }

        ParseError {
            file: self.file.clone(),
            index: error_index,
            expected: self.expected.clone(),
        }
    }

    pub fn is_finished(&self) -> bool {
        self.index == self.file.content.len()
    }

    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn get_index_backtracked(&self) -> usize {
        let mut index = self.index;

        // TODO: handle comments
        while index > 0 && is_space(self.content().as_bytes()[index - 1] as char) {
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
                Some(&self.content()[start..end])
            }
        }
    }

    pub fn as_str(&self) -> &str {
        &self.content()[self.index..]
    }

    pub fn as_char(&self) -> char {
        match self.as_str().as_bytes().first() {
            Some(byte) => *byte as char,
            None => 0 as char
        }
    }

    pub fn at(&self, index: usize) -> char {
        match self.content().as_bytes().get(self.index + index) {
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

                while self.as_char() != '\n' && self.index < self.content().len() {
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

    pub fn get_item_location(&self, start: usize) -> ItemLocation {
        ItemLocation {
            file: self.file.clone(),
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