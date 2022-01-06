use std::rc::Rc;

use crate::file_info::FileInfo;

#[derive(Debug, Clone)]
pub struct ParseError {
    pub file: Rc<FileInfo>,
    pub index: usize,
    pub expected: Vec<String>
}