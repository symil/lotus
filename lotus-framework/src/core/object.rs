use crate::{LotusValue, ReferenceWrapper};
use serializable::Serializable;

#[derive(Serializable)]
pub struct LotusObject {
    value: ReferenceWrapper<Vec<LotusValue>>
}

impl LotusObject {
    pub fn new(fields: Vec<LotusValue>) -> Self {
        Self {
            value: ReferenceWrapper::new(fields)
        }
    }

    pub fn assign(&mut self, other: &LotusValue) {
        self.value.set_outer(&other.as_object().value)
    }

    pub fn get_field(&mut self, offset: usize) -> &mut LotusValue {
        // TODO: `unwrap_unchecked`
        self.value.get_mut().get_mut(offset).unwrap()
    }
}

impl Default for LotusObject {
    fn default() -> Self {
        Self::new(vec![])
    }
}