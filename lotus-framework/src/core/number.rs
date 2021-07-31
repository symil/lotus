use serializable::Serializable;

use crate::LotusValue;

#[derive(Serializable)]
pub struct LotusNumber {
    value: f64
}

impl LotusNumber {
    pub fn new(value: f64) -> Self {
        Self { value }
    }

    pub fn assign(&mut self, other: &LotusValue) {
        self.value = other.as_number().value;
    }

    pub fn as_index(&self) -> usize {
        self.value.round() as usize
    }

    pub fn as_float(&self) -> f64 {
        self.value
    }
}

impl Default for LotusNumber {
    fn default() -> Self {
        Self::new(f64::NAN)
    }
}
