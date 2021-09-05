use std::fmt::Debug;

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

    pub fn wrap(self) -> LotusValue {
        LotusValue::number(self)
    }

    pub fn clone(&self) -> Self {
        Self { value: self.value }
    }

    pub fn as_float(&self) -> f64 {
        self.value
    }

    pub fn as_index(&self) -> usize {
        self.value.round() as usize
    }

    pub fn is_nan(&self) -> bool {
        self.value.is_nan()
    }

    pub fn to_string(&self) -> String {
        self.value.to_string()
    }

    pub fn assign(&mut self, other: &Self) {
        self.value = other.value;
    }

    pub fn equals(&self, other: &Self) -> bool {
        self.value == other.value
    }

    pub fn not_equals(&self, other: &Self) -> bool {
        self.value != other.value
    }

    pub fn less_than(&self, other: &Self) -> bool {
        self.value < other.value
    }

    pub fn less_than_or_equals(&self, other: &Self) -> bool {
        self.value <= other.value
    }

    pub fn greater_than(&self, other: &Self) -> bool {
        self.value > other.value
    }

    pub fn greater_than_or_equals(&self, other: &Self) -> bool {
        self.value >= other.value
    }

    pub fn unary_plus(&self) -> Self {
        Self::new(self.value)
    }

    pub fn unary_minus(&self) -> Self {
        Self::new(self.value * -1.)
    }

    pub fn unary_not(&self) -> bool {
        self.is_nan()
    }

    pub fn add(&self, other: &Self) -> Self {
        Self::new(self.value + other.value)
    }

    pub fn substract(&self, other: &Self) -> Self {
        Self::new(self.value - other.value)
    }

    pub fn multiply(&self, other: &Self) -> Self {
        Self::new(self.value * other.value)
    }

    pub fn divide(&self, other: &Self) -> Self {
        Self::new(self.value / other.value)
    }

    pub fn modulo(&self, other: &Self) -> Self {
        Self::new(self.value.rem_euclid(other.value))
    }
}

impl Default for LotusNumber {
    fn default() -> Self {
        Self::new(f64::NAN)
    }
}

impl Debug for LotusNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <f64 as Debug>::fmt(&self.value, f)
    }
}