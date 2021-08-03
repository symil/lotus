use std::fmt::Debug;

use crate::{LotusValue, ReferenceWrapper};
use serializable::Serializable;

#[derive(Serializable)]
pub struct LotusObject {
    value: Option<ReferenceWrapper<Vec<LotusValue>>>
}

impl LotusObject {
    pub fn new(value: Option<Vec<LotusValue>>) -> Self {
        Self {
            value: match value {
                None => None,
                Some(fields) => Some(ReferenceWrapper::new(fields))
            }
        }
    }

    pub fn wrap(self) -> LotusValue {
        LotusValue::object(self)
    }

    pub fn clone(&self) -> Self {
        Self { value: self.value.clone() }
    }

    pub fn to_string(&self, layout: &'static[&'static str]) -> String {
        todo!()
    }

    pub fn assign(&mut self, other: &LotusObject) {
        self.value = other.value.clone();
    }

    pub fn get_field(&mut self, offset: usize) -> &mut LotusValue {
        if let Some(fields_ref) = self.value.as_mut() {
            // TODO: `unwrap_unchecked`
            fields_ref.get_mut().get_mut(offset).unwrap()
        } else {
            panic!("invalid field access of null object");
        }
    }

    pub fn equals(&self, other: &Self) -> bool {
        match (&self.value, &other.value) {
            (None, None) => true,
            (None, Some(_)) => false,
            (Some(_), None) => false,
            (Some(self_value), Some(other_value)) => self_value.get_addr() == other_value.get_addr(),
        }
    }

    pub fn not_equals(&self, other: &Self) -> bool {
        !self.equals(other)
    }

    pub fn unary_not(&self) -> bool {
        self.value.is_none()
    }
}

impl Default for LotusObject {
    fn default() -> Self {
        Self::new(None)
    }
}

impl Debug for LotusObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}