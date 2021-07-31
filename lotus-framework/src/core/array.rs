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

    pub fn assign(&mut self, other: &LotusValue) {
        self.value.set_outer(&other.as_array().value)
    }

    pub fn at(&mut self, index: &LotusValue) -> &mut LotusValue {
        let i = index.as_number().as_index();

        match self.value.get_mut().get_mut(i) {
            Some(value) => value,
            None => panic!("out of bounds array access (index {}, length {})", i, self.value.get().len()),
        }
    }
}

impl Default for LotusArray {
    fn default() -> Self {
        Self::new(vec![])
    }
}