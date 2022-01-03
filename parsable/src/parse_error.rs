use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct ParseError {
    pub package_root_path: Rc<String>,
    pub file_path: Rc<String>,
    pub file_content: Rc<String>,
    pub index: usize,
    pub expected: Vec<String>
}