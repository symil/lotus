use crate::{LotusValue, ReferenceWrapper};
use serializable::Serializable;

#[derive(Serializable)]
pub struct LotusArray {
    value: ReferenceWrapper<Vec<LotusValue>>
}

impl LotusArray {
    pub fn new(items: Vec<LotusValue>) -> Self {
        Self {
            value: ReferenceWrapper::new(items)
        }
    }

    pub fn wrap(self) -> LotusValue {
        LotusValue::array(self)
    }

    pub fn clone(&self) -> Self {
        Self { value: self.value.clone() }
    }

    fn items(&self) -> &Vec<LotusValue> {
        self.value.get()
    }

    pub fn at(&mut self, index: usize) -> &mut LotusValue {
        match self.value.get_mut().get_mut(index) {
            Some(value) => value,
            None => panic!("out of bounds array access (index {}, length {})", index, self.value.get().len()),
        }
    }

    pub fn first(&mut self) -> &mut LotusValue {
        self.at(0)
    }

    pub fn last(&mut self) -> &mut LotusValue {
        self.at(self.len() - 1)
    }

    pub fn push(&mut self, value: LotusValue) {
        self.value.get_mut().push(value);
    }

    pub fn len(&self) -> usize {
        self.value.get().len()
    }

    pub fn assign(&mut self, other: &LotusArray) {
        self.value.set_outer(&other.value)
    }

    pub fn equals(&self, other: &Self) -> bool {
        let items_1 = self.items();
        let items_2 = other.items();

        if items_1.len() != items_2.len() {
            return false;
        }

        for i in 0..items_1.len() {
            if !items_1[i].equals(&items_2[i]) {
                return false;
            }
        }

        true
    }

    pub fn not_equals(&self, other: &Self) -> bool {
        !self.equals(other)
    }

    pub fn unary_not(&self) -> bool {
        self.items().is_empty()
    }

    pub fn add(&self, other: &Self) -> Self {
        let mut result = Vec::with_capacity(self.items().len() + other.items().len());

        result.extend_from_slice(&self.items());
        result.extend_from_slice(&other.items());

        Self::new(result)
    }
}

impl Default for LotusArray {
    fn default() -> Self {
        Self::new(vec![])
    }
}