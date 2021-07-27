use parsable::parsable;

#[parsable(name="number")]
pub struct NumberLiteral {
    pub value: f64,
}

impl std::ops::Deref for NumberLiteral {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::ops::DerefMut for NumberLiteral {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }    
}