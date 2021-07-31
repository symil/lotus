use serializable::Serializable;

use crate::LotusValue;

#[derive(Serializable)]
pub struct LotusBoolean {
    value: bool
}

impl LotusBoolean {
    pub fn new(value: bool) -> Self {
        Self { value }
    }

    pub fn assign(&mut self, other: &LotusValue) {
        self.value = other.as_boolean().value;
    }

    pub fn is_true(&self) -> bool {
        self.value
    }
}

impl Default for LotusBoolean {
    fn default() -> Self {
        Self::new(false)
    }
}
