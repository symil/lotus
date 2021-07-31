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

    pub fn wrap(self) -> LotusValue {
        LotusValue::boolean(self)
    }

    pub fn clone(&self) -> Self {
        Self { value: self.value }
    }

    pub fn as_bool(&self) -> bool {
        self.value
    }

    pub fn assign(&mut self, other: &LotusBoolean) {
        self.value = other.value;
    }

    pub fn equals(&self, other: &LotusBoolean) -> bool {
        self.value == other.value
    }
}

impl Default for LotusBoolean {
    fn default() -> Self {
        Self::new(false)
    }
}
