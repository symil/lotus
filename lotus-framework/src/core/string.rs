use std::rc::Rc;

use serializable::Serializable;

use crate::LotusValue;

#[derive(Serializable)]
pub struct LotusString {
    value: Rc<Vec<char>>
}

impl LotusString {
    pub fn new(value: &str) -> Self {
        Self {
            value: Rc::new(value.chars().collect())
        }
    }

    pub fn assign(&mut self, other: &LotusValue) {
        self.value = Rc::clone(&other.as_string().value);
    }
}

impl Default for LotusString {
    fn default() -> Self {
        Self::new("")
    }
}
