use std::rc::Rc;

use serializable::Serializable;

use crate::LotusValue;

#[derive(Serializable)]
pub struct LotusString {
    value: Rc<Vec<char>>
}

impl LotusString {
    pub fn new(value: Vec<char>) -> Self {
        Self {
            value: Rc::new(value)
        }
    }

    pub fn wrap(self) -> LotusValue {
        LotusValue::string(self)
    }

    pub fn clone(&self) -> Self {
        Self { value: Rc::clone(&self.value) }
    }

    pub fn to_string(&self) -> String {
        self.value.iter().collect()
    }

    pub fn at(&self, index: usize) -> char {
        match self.value.get(index) {
            Some(value) => *value,
            None => panic!("out of bounds string access (index {}, length {})", index, self.value.len()),
        }
    }

    pub fn assign(&mut self, other: &LotusString) {
        self.value = Rc::clone(&other.value);
    }

    pub fn equals(&self, other: &Self) -> bool {
        if self.value.len() != other.value.len() {
            return false;
        }

        for i in 0..self.value.len() {
            if self.value[i] != other.value[i] {
                return false;
            }
        }

        true
    }

    pub fn not_equals(&self, other: &Self) -> bool {
        !self.equals(other)
    }

    pub fn unary_not(&self) -> bool {
        self.value.is_empty()
    }

    pub fn add(&self, other: &Self) -> Self {
        let mut result = Vec::with_capacity(self.value.len() + other.value.len());

        result.extend_from_slice(&self.value);
        result.extend_from_slice(&other.value);

        Self::new(result)
    }
}

impl Default for LotusString {
    fn default() -> Self {
        Self::new(vec![])
    }
}
